import SwiftRs
import AppKit
import Foundation

@_cdecl("get_file_thumbnail_base64")
func getFileThumbnailBase64(path: SRString) -> SRString {
    let path = path.to_string();
    
    let image = NSWorkspace.shared.icon(forFile: path)
    let bitmap = NSBitmapImageRep(data: image.tiffRepresentation!)!.representation(using: .png, properties: [:])!
    
    return SRString(bitmap.base64EncodedString())
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
