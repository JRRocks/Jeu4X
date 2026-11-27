pluginManagement {
    repositories {
        google()
        // Miroir de Maven Central chez Google : Maven Central répond 429 aux rafales,
        // en session comme en CI (exécution n° 8). Maven Central reste en repli.
        maven("https://maven-central.storage-download.googleapis.com/maven2")
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        maven("https://maven-central.storage-download.googleapis.com/maven2")
        mavenCentral()
    }
}
rootProject.name = "Jeu4X"
include(":app")
