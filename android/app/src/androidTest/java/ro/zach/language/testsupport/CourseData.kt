package ro.zach.language.testsupport

import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONArray
import org.json.JSONObject
import java.io.InputStream

/** The repository's course data, which the test APK carries as assets. */
object CourseData {
    const val COURSE_ID: String = "pt_br.en.foundations"
    const val PRACTICED_UNIT_ID: String = "a1-01-greetings-introductions"

    fun readCurriculum(): JSONObject {
        return readJson("courses/$COURSE_ID/curriculum.json")
    }

    fun readUnit(unitId: String): JSONObject {
        return readJson("courses/$COURSE_ID/units/$unitId.json")
    }

    /** The unit's exercises the app offers, which leaves out open responses. */
    fun readExercises(unitId: String): JSONArray {
        val allExercises: JSONArray = readUnit(unitId).getJSONArray("exercises")
        val offeredExercises: JSONArray = JSONArray()

        for (index in 0 until allExercises.length()) {
            val exercise: JSONObject = allExercises.getJSONObject(index)

            if (exercise.getString("type") != "open_response") {
                offeredExercises.put(exercise)
            }
        }

        return offeredExercises
    }

    fun readCourseTitle(): String {
        return readCurriculum().getString("title")
    }

    private fun readJson(assetPath: String): JSONObject {
        val text: String = InstrumentationRegistry.getInstrumentation().context.assets.open(assetPath).use { input: InputStream ->
            input.bufferedReader().readText()
        }

        return JSONObject(text)
    }
}
