import SwiftRs
import AppKit
import Foundation
import QuickLookThumbnailing

@_cdecl("get_file_icon_base64")
func getFileIconBase64(path: SRString) -> SRString {
    let path = path.to_string();
    
    let image = NSWorkspace.shared.icon(forFile: path)
    let bitmap = NSBitmapImageRep(data: image.tiffRepresentation!)!.representation(using: .png, properties: [:])!
    
    return SRString(bitmap.base64EncodedString())
}

@_cdecl("get_file_thumbnail_base64")
func getFileThumbnailBase64(for path: SRString) -> SRString {
    let size = CGSize(width: 320, height: 320)
    let scale = AppKit.NSScreen.main?.backingScaleFactor
    let fileURL = URL(fileURLWithPath: path.to_string())

    let request = QLThumbnailGenerator
        .Request(fileAt: fileURL, size: size, scale: scale!,
                 representationTypes: .lowQualityThumbnail)

    request.iconMode = true
    let semaphore = DispatchSemaphore(value: 0)
    var ret = SRString()
    
    QLThumbnailGenerator.shared.generateBestRepresentation(for: request)
    { (thumbnail, error) in
        if thumbnail == nil || error != nil {
            // Handle the error case gracefully.
        } else {
            // Display the thumbnail that you created.
            let image = thumbnail?.nsImage
            let bitmap = NSBitmapImageRep(data: image!.tiffRepresentation!)!.representation(using: .png, properties: [:])!
            ret = SRString(bitmap.base64EncodedString())
        }
        semaphore.signal()
    }
    
    semaphore.wait()
    return ret
}

@_cdecl("paste")
func paste() -> SRString {
    let applescript = """
        tell application "System Events" 
        keystroke "v" using {command down} 
        end tell
    """
    var error: NSDictionary?
    if let scriptObject = NSAppleScript(source: applescript) {
        if let outputString = scriptObject.executeAndReturnError(&error).stringValue {
            return SRString(outputString)
        } else if (error != nil) {
            return SRString(String("error: \(error)"))
        }
    }
    return SRString();
}
