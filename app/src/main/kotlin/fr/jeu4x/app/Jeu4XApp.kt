package fr.jeu4x.app

import android.app.Application
import android.content.Context
import fr.jeu4x.app.crash.CrashReporter

class Jeu4XApp : Application() {
    override fun attachBaseContext(base: Context) {
        super.attachBaseContext(base)
        if (CrashReporter.isCrashProcess(this)) return
        // En tout premier : à partir d'ici, une exception finit sur l'écran de rapport
        CrashReporter.install(this)
    }

    override fun onCreate() {
        super.onCreate()
        if (CrashReporter.isCrashProcess(this)) return
        CrashReporter.collectNativeCrash(this)
    }
}
