package io.github.positive_mahjong.tw0hank0

import android.app.Activity
import android.os.Bundle
import android.util.Log
import androidx.appcompat.app.AppCompatActivity

/**
 * Positive Mahjong 主活動類別
 * 負責載入 Rust 函式庫並啟動應用邏輯
 */
class MainActivity : AppCompatActivity() {

    companion object {
        private const val TAG: String = "PMJ_MainActivity"
        
        // 載入 Rust 共享庫
        // 對應 Cargo.toml 中的 name = "pmj_client" -> 生成 libpmj_client.so
        init {
            System.loadLibrary("pmj_client")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        Log.d(TAG, "MainActivity 已建立，準備初始化 Rust 引擎...")

        // 呼叫 Rust 端的初始化函數
        // 該函數必須在 Rust 中定義為: pub extern "C" fn java_init_activity(...)
        try {
            initializeRustEngine(this)
            Log.d(TAG, "Rust 引擎初始化成功")
        } catch (e: UnsatisfiedLinkError) {
            Log.e(TAG, "無法找到 Rust 初始化函數，請檢查 lib.rs 中的 JNI 綁定", e)
        } catch (e: Exception) {
            Log.e(TAG, "初始化過程中發生未知錯誤", e)
        }

        // 注意：
        // 若 Slint 使用 android-activity crate 的完整生命週期管理，
        // 通常會直接接管視窗渲染。在此模式下，我們可能不需要設置 setContentView，
        // 因為 Rust 側會繪製到 SurfaceView 或 NativeWindow 上。
        // 若您需要覆蓋原生 Android UI，可在此處 setContentView(R.layout.activity_main)
    }

    override fun onResume() {
        super.onResume()
        // 通知 Rust 應用進入前台
        notifyRustResumed()
    }

    override fun onPause() {
        super.onPause()
        // 通知 Rust 應用進入背景
        notifyRustPaused()
    }

    override fun onDestroy() {
        super.onDestroy()
        // 通知 Rust 應用即將銷毀
        notifyRustDestroyed()
    }

    // === JNI native 方法宣告 ===
    // 這些方法必須對應 Rust 側的 #[no_mangle] pub extern "C" 函數
    
    /**
     * 初始化 Rust 引擎並傳遞 Activity 上下文
     * @param activity 當前的 Android Activity 實例
     */
    private external fun initializeRustEngine(activity: Activity)

    /**
     * 通知 Rust 應用已恢復運行
     */
    private external fun notifyRustResumed()

    /**
     * 通知 Rust 應用已暫停
     */
    private external fun notifyRustPaused()

    /**
     * 通知 Rust 應用已銷毀
     */
    private external fun notifyRustDestroyed()
}