package ro.zach.language.testsupport

import androidx.test.platform.app.InstrumentationRegistry
import org.json.JSONArray
import org.json.JSONObject
import java.io.InputStream

private const val OPEN_RESPONSE_TYPE: String = "open_response"

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

    /** The unit's exercises other than its open responses, which the quiz asks. */
    fun readQuizExercises(unitId: String): JSONArray {
        return readExercises(unitId) { exercise -> exercise.getString("type") != OPEN_RESPONSE_TYPE }
    }

    fun readOpenResponseExercises(unitId: String): JSONArray {
        return readExercises(unitId) { exercise -> exercise.getString("type") == OPEN_RESPONSE_TYPE }
    }

    fun readCourseTitle(): String {
        return readCurriculum().getString("title")
    }

    private fun readExercises(unitId: String, isIncluded: (JSONObject) -> Boolean): JSONArray {
        val allExercises: JSONArray = readUnit(unitId).getJSONArray("exercises")
        val includedExercises: JSONArray = JSONArray()

        for (index in 0 until allExercises.length()) {
            val exercise: JSONObject = allExercises.getJSONObject(index)

            if (isIncluded(exercise)) {
                includedExercises.put(exercise)
            }
        }

        return includedExercises
    }

    private fun readJson(assetPath: String): JSONObject {
        val text: String = InstrumentationRegistry.getInstrumentation().context.assets.open(assetPath).use { input: InputStream ->
            input.bufferedReader().readText()
        }

        return JSONObject(text)
    }
}
