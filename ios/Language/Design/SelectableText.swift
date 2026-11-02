import SwiftUI
import UIKit

/// Body text that can be selected by range, not only copied whole.
struct SelectableText: UIViewRepresentable {
    let text: String

    func makeUIView(context: Context) -> UITextView {
        let textView: UITextView = UITextView()

        textView.isEditable = false
        textView.isSelectable = true
        textView.isScrollEnabled = false
        textView.backgroundColor = .clear
        textView.textContainerInset = .zero
        textView.textContainer.lineFragmentPadding = 0
        textView.setContentCompressionResistancePriority(.defaultLow, for: .horizontal)
        textView.adjustsFontForContentSizeCategory = false

        return textView
    }

    func updateUIView(_ textView: UITextView, context: Context) {
        textView.attributedText = Self.attributedText(text)
        textView.accessibilityLabel = text
    }

    func sizeThatFits(_ proposal: ProposedViewSize, uiView textView: UITextView, context: Context) -> CGSize? {
        guard let width = proposal.width
        else {
            return nil
        }

        let fitted: CGSize = textView.sizeThatFits(CGSize(width: width, height: .greatestFiniteMagnitude))

        return CGSize(width: width, height: fitted.height)
    }

    private static func attributedText(_ text: String) -> NSAttributedString {
        let paragraphStyle: NSMutableParagraphStyle = NSMutableParagraphStyle()
        paragraphStyle.lineSpacing = .bodyLineSpacing

        return NSAttributedString(
            string: text,
            attributes: [
                .font: UIFont.systemFont(ofSize: .bodyTextSize),
                .foregroundColor: UIColor(Color.ink),
                .paragraphStyle: paragraphStyle,
            ]
        )
    }
}
