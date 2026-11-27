import java.io.FileInputStream
import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("org.jetbrains.kotlin.plugin.compose")
}

// Version qui monte seule : le numéro d'exécution de la CI (unique et croissant pour ce
// workflow). Hors CI : 1, version « locale » jamais livrée.
val numeroVersion = providers.environmentVariable("JEU4X_VERSION_CODE").orNull?.toInt() ?: 1
val nomVersion = "0.1.$numeroVersion"

// Clé de signature release : jamais versionnée (dépôt public). Absente : APK release non
// signé, que la CI refuse de livrer.
val proprietesRelease = file("keystore-release/keystore.properties")
val signatureReleaseDisponible = proprietesRelease.exists()

val dossierMoteur = rootProject.file("moteur")
val sortieJniAndroid = layout.buildDirectory.dir("rustJniLibs")

android {
    namespace = "fr.jeu4x.app"
    compileSdk = 35
    ndkVersion = "27.2.12479018"

    defaultConfig {
        // Identifiant définitif : il ne change jamais, sinon la mise à jour par-dessus casse.
        applicationId = "fr.jeu4x.app"
        minSdk = 26
        targetSdk = 35
        versionCode = numeroVersion
        versionName = nomVersion
        // Le commit de l'APK, écrit dans les rapports de plantage : connu en CI, « local » sinon
        val commit = providers.environmentVariable("GITHUB_SHA").orNull?.take(7) ?: "local"
        buildConfigField("String", "GIT_SHA", "\"$commit\"")
        // Plantage provoqué à la demande, pour vérifier le rapport à l'émulateur : essai seulement.
        buildConfigField("boolean", "PLANTAGE_DE_TEST", "false")
        ndk { abiFilters += listOf("arm64-v8a") }
    }

    signingConfigs {
        // Clé d'essai publique, versionnée exprès (app/essai/LISEZMOI.md) : jamais pour le vrai jeu.
        create("essai") {
            storeFile = file("essai/cle-essai.keystore")
            storePassword = "essai-public"
            keyAlias = "essai"
            keyPassword = "essai-public"
        }
        if (signatureReleaseDisponible) {
            create("release") {
                val proprietes = Properties().apply { load(FileInputStream(proprietesRelease)) }
                storeFile = file("keystore-release/${proprietes.getProperty("storeFile")}")
                keyAlias = proprietes.getProperty("keyAlias")
                // Mots de passe par l'environnement : `Properties.load` corromprait un `\`.
                storePassword = System.getenv("SIGNATURE_STORE_PASSWORD") ?: proprietes.getProperty("storePassword")
                keyPassword = System.getenv("SIGNATURE_KEY_PASSWORD") ?: proprietes.getProperty("keyPassword")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            if (signatureReleaseDisponible) {
                signingConfig = signingConfigs.getByName("release")
            }
        }
        // Version d'essai (D-17) : s'installe à côté du vrai jeu, nom « Jeu4X essai ».
        create("essai") {
            initWith(getByName("release"))
            applicationIdSuffix = ".essai"
            signingConfig = signingConfigs.getByName("essai")
            matchingFallbacks += listOf("release")
            // x86_64 en plus : l'essai passe d'abord par l'émulateur de la CI (D-19).
            ndk { abiFilters += listOf("x86_64") }
            buildConfigField("boolean", "PLANTAGE_DE_TEST", "true")
        }
    }

    sourceSets["main"].jniLibs.srcDir(sortieJniAndroid)

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
    buildFeatures {
        compose = true
        buildConfig = true
    }
    testOptions { unitTests.isIncludeAndroidResources = true }
}

androidComponents {
    // ui-test-manifest n'existe qu'en debug : pas de tests unitaires en release.
    beforeVariants(selector().withBuildType("release")) { it.enableUnitTest = false }
    beforeVariants(selector().withBuildType("essai")) { it.enableUnitTest = false }
}

// Bibliothèque Rust pour le téléphone (arm64-v8a), compilée par cargo-ndk.
val cargoAndroid by tasks.registering(Exec::class) {
    description = "Compile le moteur Rust pour Android (arm64-v8a ; x86_64 pour l'émulateur)"
    workingDir = dossierMoteur
    inputs.dir(dossierMoteur.resolve("regles/src"))
    inputs.dir(dossierMoteur.resolve("android/src"))
    outputs.dir(sortieJniAndroid)
    doFirst {
        environment("ANDROID_NDK_HOME", android.ndkDirectory.absolutePath)
    }
    commandLine(
        "cargo", "ndk", "-t", "arm64-v8a", "-t", "x86_64", "--platform", "26",
        "-o", sortieJniAndroid.get().asFile.absolutePath,
        "build", "--release", "-p", "jeu4x_android",
    )
}

// La même bibliothèque compilée pour le PC : chargée par les tests JVM (test de la frontière).
val cargoHote by tasks.registering(Exec::class) {
    description = "Compile le moteur Rust pour la machine de test"
    workingDir = dossierMoteur
    inputs.dir(dossierMoteur.resolve("regles/src"))
    inputs.dir(dossierMoteur.resolve("android/src"))
    outputs.dir(dossierMoteur.resolve("target/release"))
    commandLine("cargo", "build", "--release", "-p", "jeu4x_android")
}

tasks.matching { it.name.startsWith("merge") && it.name.endsWith("JniLibFolders") }.configureEach {
    dependsOn(cargoAndroid)
}

tasks.withType<Test>().configureEach {
    dependsOn(cargoHote)
    systemProperty("java.library.path", dossierMoteur.resolve("target/release").absolutePath)
    forkEvery = 1
}

dependencies {
    val composeBom = platform("androidx.compose:compose-bom:2024.10.01")
    implementation(composeBom)
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.activity:activity-compose:1.9.3")
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.material3:material3")

    testImplementation(composeBom)
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.jetbrains.kotlin:kotlin-test-junit:2.0.21")
    testImplementation("org.robolectric:robolectric:4.14.1")
    testImplementation("androidx.test:core:1.6.1")
    testImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}
