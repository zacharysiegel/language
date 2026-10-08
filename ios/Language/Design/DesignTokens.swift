import SwiftUI

extension Color {
    static let paper = Color(white: 1.0)
    static let ink = Color(white: 0.0)
    static let accentActive = Color(red: 0.835, green: 0.0, blue: 0.0) // #d50000
    static let accentHover = Color(red: 0.71, green: 0.0, blue: 0.0) // #b50000
    static let accentLink = Color(red: 0.0, green: 0.314, blue: 1.0) // #0050ff
    static let rule = Color(white: 0.831) // #d4d4d4
    static let inkMuted = Color(white: 0.333) // #555555
    static let goodText = Color(red: 0.0, green: 0.42, blue: 0.13)
}

extension CGFloat {
    static let spaceXs: CGFloat = 4
    static let spaceSm: CGFloat = 8
    static let spaceMd: CGFloat = 16
    static let spaceLg: CGFloat = 32
    static let spaceXl: CGFloat = 48
    static let hairline: CGFloat = 1
    static let buttonIconSize: CGFloat = 30
    static let measure: CGFloat = 640
}

/* The web's type scale, one or two points larger for a phone's reading distance. */
extension Font {
    static let bodyText = Font.system(size: 15)
    static let dataText = Font.system(size: 14).monospacedDigit()
    static let labelText = Font.system(size: 11, weight: .semibold)
    static let captionText = Font.system(size: 12)
    static let titleText = Font.system(size: 17, weight: .semibold)
    static let headingText = Font.system(size: 22, weight: .semibold)
    static let figureText = Font.system(size: 38, weight: .semibold).monospacedDigit()
    static let targetText = Font.system(size: 16, weight: .medium)
}
