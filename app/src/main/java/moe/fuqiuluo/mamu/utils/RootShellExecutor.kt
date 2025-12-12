package moe.fuqiuluo.mamu.utils

import android.content.Context
import android.util.Log
import com.topjohnwu.superuser.Shell
import moe.fuqiuluo.mamu.R
import java.util.concurrent.TimeoutException

/**
 * Root Shell 执行结果
 */
sealed class ShellResult {
    data class Success(val output: String, val exitCode: Int = 0) : ShellResult()
    data class Error(val message: String, val exitCode: Int = -1) : ShellResult()
    data class Timeout(val duration: Long) : ShellResult()
}

/**
 * Root Shell 执行器
 * 使用 libsu 实现，提供稳定可靠的 root shell 执行能力
 */
object RootShellExecutor {
    private const val TAG = "RootShellExecutor"

    init {
        // 配置 libsu
        // 不使用 FLAG_REDIRECT_STDERR，分开捕获 stdout 和 stderr
        Shell.enableVerboseLogging = false
        Shell.setDefaultBuilder(
            Shell.Builder.create()
                .setInitializers(ShellInit::class.java)
                .setFlags(Shell.FLAG_MOUNT_MASTER)
                .setTimeout(10)
        )
    }

    class ShellInit : Shell.Initializer() {
        override fun onInit(context: Context, shell: Shell): Boolean {
            val bashrc = context.resources.openRawResource(R.raw.bashrc)
            shell.newJob().add(bashrc).exec()
            return true
        }
    }

    /**
     * 使用自定义 su 命令配置 Shell
     */
    fun getShellBuilder(suCmd: String): Shell.Builder {
        return Shell.Builder.create()
            .setFlags(Shell.FLAG_MOUNT_MASTER)
            .setTimeout(10)
            .setCommands(suCmd)
    }

    /**
     * 一次性执行命令
     * @param suCmd su 命令路径（如 "su" 或自定义路径）
     * @param command 要执行的命令
     * @param timeoutMs 超时时间（毫秒），libsu 内部会处理超时
     */
    fun exec(
        suCmd: String,
        command: String,
        timeoutMs: Long = 5000L
    ): ShellResult {
        return try {
            val result = if (suCmd == RootConfigManager.DEFAULT_ROOT_COMMAND) {
                // 使用默认 shell
                Log.d(TAG, "Shell.cmd($command)")
                Shell.cmd(command).exec()
            } else {
                // 使用自定义 su 命令创建新 shell
                val shell = getShellBuilder(suCmd).build()
                Log.d(TAG, "Shell.cmd2($command)")
                shell.newJob().add(command).exec()
            }

            val stdout = result.out.joinToString("\n")
            val stderr = result.err.joinToString("\n")
            val exitCode = result.code

            // 合并 stdout 和 stderr，确保都能被解析
            val combinedOutput = buildString {
                if (stdout.isNotEmpty()) append(stdout)
                if (stderr.isNotEmpty()) {
                    if (isNotEmpty()) append("\n")
                    append(stderr)
                }
            }

            Log.d(TAG, "Command result - exitCode: $exitCode, stdout: $stdout, stderr: $stderr")

            if (result.isSuccess) {
                ShellResult.Success(combinedOutput, exitCode)
            } else {
                ShellResult.Error(
                    combinedOutput.ifEmpty { "Command failed with exit code $exitCode" },
                    exitCode
                )
            }
        } catch (e: Exception) {
            Log.e(TAG, "Failed to execute command: $command", e)
            ShellResult.Error(e.message ?: "Unknown error", -1)
        }
    }

    /**
     * 批量一次性执行命令
     */
    fun execBatch(
        suCmd: String,
        commands: List<String>,
        timeoutMs: Long = 5000L
    ): List<ShellResult> {
        return commands.map { exec(suCmd, it, timeoutMs) }
    }

    /**
     * Fire and forget - 不等待结果
     */
    fun execNoWait(suCmd: String, command: String) {
        try {
            if (suCmd == RootConfigManager.DEFAULT_ROOT_COMMAND) {
                Shell.cmd(command).submit()
            } else {
                val shell = getShellBuilder(suCmd).build()
                shell.newJob().add(command).submit()
            }
        } catch (e: Exception) {
            Log.e(TAG, "Failed to execute command (no wait): $command", e)
        }
    }

    /**
     * DSL 风格：使用 libsu Shell 执行多条命令
     * 注意：libsu 使用全局 Shell，无需手动管理生命周期
     */
    inline fun <T> withPersistentShell(
        suCmd: String,
        block: Shell.() -> T
    ): T {
        val shell = if (suCmd == RootConfigManager.DEFAULT_ROOT_COMMAND) {
            Shell.getShell()
        } else {
            getShellBuilder(suCmd).build()
        }
        return shell.block()
    }
}

/**
 * Shell 扩展：异步执行命令
 */
fun Shell.executeAsync(
    suCmd: String,
    command: String,
    callback: (ShellResult) -> Unit
) {
    newJob().add(command).submit { result ->
        val stdout = result.out.joinToString("\n")
        val stderr = result.err.joinToString("\n")
        val exitCode = result.code

        val combinedOutput = buildString {
            if (stdout.isNotEmpty()) append(stdout)
            if (stderr.isNotEmpty()) {
                if (isNotEmpty()) append("\n")
                append(stderr)
            }
        }

        val shellResult = if (result.isSuccess) {
            ShellResult.Success(combinedOutput, exitCode)
        } else {
            ShellResult.Error(
                combinedOutput.ifEmpty { "Command failed with exit code $exitCode" },
                exitCode
            )
        }
        callback(shellResult)
    }
}

/**
 * String 扩展：直接作为 root 命令执行
 */
fun String.asRootCommand(suCmd: String, timeoutMs: Long = 5000L): ShellResult =
    RootShellExecutor.exec(suCmd, this, timeoutMs)

/**
 * ShellResult 扩展：成功时执行回调
 */
inline fun ShellResult.onSuccess(block: (String) -> Unit): ShellResult {
    if (this is ShellResult.Success) block(output)
    return this
}

/**
 * ShellResult 扩展：失败时执行回调
 */
inline fun ShellResult.onError(block: (String) -> Unit): ShellResult {
    if (this is ShellResult.Error) block(message)
    return this
}

/**
 * ShellResult 扩展：超时时执行回调
 */
inline fun ShellResult.onTimeout(block: (Long) -> Unit): ShellResult {
    if (this is ShellResult.Timeout) block(duration)
    return this
}

/**
 * ShellResult 扩展：获取结果或返回 null
 */
fun ShellResult.getOrNull(): String? =
    (this as? ShellResult.Success)?.output

/**
 * ShellResult 扩展：获取结果或返回默认值
 */
fun ShellResult.getOrDefault(default: String): String =
    (this as? ShellResult.Success)?.output ?: default

/**
 * ShellResult 扩展：获取结果或抛出异常
 */
fun ShellResult.getOrThrow(): String = when (this) {
    is ShellResult.Success -> output
    is ShellResult.Error -> throw RuntimeException("Command failed: $message (exit code: $exitCode)")
    is ShellResult.Timeout -> throw TimeoutException("Command timeout after ${duration}ms")
}

/**
 * ShellResult 扩展：判断是否成功
 */
fun ShellResult.isSuccess(): Boolean = this is ShellResult.Success

/**
 * ShellResult 扩展：判断是否失败
 */
fun ShellResult.isError(): Boolean = this is ShellResult.Error

/**
 * ShellResult 扩展：判断是否超时
 */
fun ShellResult.isTimeout(): Boolean = this is ShellResult.Timeout
