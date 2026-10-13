package ro.zach.language.design

import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

/* The only place colors, spaces and type sizes are written; the values follow web/style/_tokens.scss. */

object LanguageColors {
    val paper: Color = Color(0xFFFFFFFF)
    val ink: Color = Color(0xFF000000)
    val accentActive: Color = Color(0xFFD50000)
    val accentHover: Color = Color(0xFFB50000)
    val accentLink: Color = Color(0xFF0050FF)
    val rule: Color = Color(0xFFD4D4D4)
    val accentTint: Color = Color(0xFFFBE6E6)
    val inkMuted: Color = Color(0xFF555555)
    val goodText: Color = Color(0xFF006B21)
}

object LanguageSpacing {
    val spaceXs: Dp = 4.dp
    val spaceSm: Dp = 8.dp
    val spaceMd: Dp = 16.dp
    val spaceLg: Dp = 32.dp
    val spaceXl: Dp = 48.dp
    val hairline: Dp = 1.dp
    val buttonIconSize: Dp = 30.dp
    val measure: Dp = 640.dp
    val boxShadowOffset: Dp = 4.dp
    val smallBoxShadowOffset: Dp = 2.dp
    val floatingTipMaximumWidth: Dp = 320.dp
    val indexRailWidth: Dp = 20.dp
    val indexRailEdgeInset: Dp = 6.dp
    val indexRailEntryMaximumHeight: Dp = 21.dp
    val tableFieldLabelWidth: Dp = 96.dp
    val tableCellMinimumWidth: Dp = 80.dp
    val tableCellVerticalPadding: Dp = 6.dp
    /** Between the stacked lines of one phrase: its target, translation and gloss. */
    val phraseLineGap: Dp = 2.dp
    val checkBoxSize: Dp = 18.dp
    val tagHorizontalPadding: Dp = 6.dp
    val tagVerticalPadding: Dp = 2.dp
    val progressBarHeight: Dp = 6.dp
    val answerAreaMinimumHeight: Dp = 44.dp
    val answerAreaDashLength: Dp = 3.dp
    val selectedBorderWidth: Dp = 2.dp
    val topBarSideWidth: Dp = 128.dp
    /** Below this window width a page lays out for a phone held upright. */
    val compactWidthThreshold: Dp = 600.dp
    val transferTextHeight: Dp = 140.dp
}

/* The web's type scale, one or two points larger for a phone's reading distance. */
object LanguageType {
    private const val TABULAR_FIGURES: String = "tnum"

    val bodyText: TextStyle = TextStyle(fontSize = 15.sp)
    val dataText: TextStyle = TextStyle(fontSize = 14.sp, fontFeatureSettings = TABULAR_FIGURES)
    val labelText: TextStyle = TextStyle(fontSize = 11.sp, fontWeight = FontWeight.SemiBold)
    val labelFigureText: TextStyle = TextStyle(fontSize = 11.sp, fontWeight = FontWeight.SemiBold, fontFeatureSettings = TABULAR_FIGURES)
    val captionText: TextStyle = TextStyle(fontSize = 12.sp)
    val captionFigureText: TextStyle = TextStyle(fontSize = 12.sp, fontFeatureSettings = TABULAR_FIGURES)
    val indexEntryText: TextStyle = TextStyle(fontSize = 10.sp, fontFeatureSettings = TABULAR_FIGURES)
    val titleText: TextStyle = TextStyle(fontSize = 17.sp, fontWeight = FontWeight.SemiBold)
    val headingText: TextStyle = TextStyle(fontSize = 22.sp, fontWeight = FontWeight.SemiBold)
    val figureText: TextStyle = TextStyle(fontSize = 38.sp, fontWeight = FontWeight.SemiBold, fontFeatureSettings = TABULAR_FIGURES)
    val targetText: TextStyle = TextStyle(fontSize = 16.sp, fontWeight = FontWeight.Medium)
    val glyphText: TextStyle = TextStyle(fontSize = 11.sp, fontWeight = FontWeight.Bold)
    val transferText: TextStyle = TextStyle(fontSize = 11.sp, fontFamily = FontFamily.Monospace)
    val labelTracking: TextUnit = 0.6.sp
    val tagTracking: TextUnit = 0.4.sp
    val paragraphLineHeight: TextUnit = 21.sp
}
