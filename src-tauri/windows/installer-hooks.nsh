!macro NSIS_HOOK_POSTINSTALL
  ; Register FeatherMark with Windows Default Apps without taking ownership of
  ; any extension. Windows requires the user to confirm defaults in Settings.
  WriteRegStr HKCU "Software\Classes\FeatherMark.Markdown" "" "Markdown document"
  WriteRegStr HKCU "Software\Classes\FeatherMark.Markdown" "FriendlyTypeName" "Markdown document"
  WriteRegStr HKCU "Software\Classes\FeatherMark.Markdown\DefaultIcon" "" "$INSTDIR\feathermark.exe,0"
  WriteRegStr HKCU "Software\Classes\FeatherMark.Markdown\shell\open\command" "" '$\"$INSTDIR\feathermark.exe$\" $\"%1$\"'

  ; Make FeatherMark an explicit candidate in the Windows Open with chooser.
  ; SupportedTypes limits the entry to Markdown instead of advertising it for
  ; unrelated files. OpenWithProgids keeps it discoverable for both extensions.
  WriteRegStr HKCU "Software\Classes\Applications\feathermark.exe" "FriendlyAppName" "FeatherMark"
  WriteRegStr HKCU "Software\Classes\Applications\feathermark.exe\DefaultIcon" "" "$INSTDIR\feathermark.exe,0"
  WriteRegStr HKCU "Software\Classes\Applications\feathermark.exe\SupportedTypes" ".md" ""
  WriteRegStr HKCU "Software\Classes\Applications\feathermark.exe\SupportedTypes" ".markdown" ""
  WriteRegStr HKCU "Software\Classes\Applications\feathermark.exe\shell\open\command" "" '$\"$INSTDIR\feathermark.exe$\" $\"%1$\"'
  WriteRegStr HKCU "Software\Classes\.md\OpenWithProgids" "FeatherMark.Markdown" ""
  WriteRegStr HKCU "Software\Classes\.markdown\OpenWithProgids" "FeatherMark.Markdown" ""

  ; Provide a stable secondary verb even when another application remains the
  ; default. Windows 11 may place classic desktop verbs under Show more options.
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open" "" "Open with FeatherMark"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open" "MUIVerb" "Open with FeatherMark"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open" "Icon" "$INSTDIR\feathermark.exe,0"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open" "MultiSelectModel" "Document"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open\command" "" '$\"$INSTDIR\feathermark.exe$\" $\"%1$\"'
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open" "" "Open with FeatherMark"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open" "MUIVerb" "Open with FeatherMark"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open" "Icon" "$INSTDIR\feathermark.exe,0"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open" "MultiSelectModel" "Document"
  WriteRegStr HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open\command" "" '$\"$INSTDIR\feathermark.exe$\" $\"%1$\"'

  WriteRegStr HKCU "Software\FeatherMark\Capabilities" "ApplicationName" "FeatherMark"
  WriteRegStr HKCU "Software\FeatherMark\Capabilities" "ApplicationDescription" "A lightweight Markdown viewer"
  WriteRegStr HKCU "Software\FeatherMark\Capabilities" "ApplicationIcon" "$INSTDIR\feathermark.exe,0"
  WriteRegStr HKCU "Software\FeatherMark\Capabilities\FileAssociations" ".md" "FeatherMark.Markdown"
  WriteRegStr HKCU "Software\FeatherMark\Capabilities\FileAssociations" ".markdown" "FeatherMark.Markdown"
  WriteRegStr HKCU "Software\RegisteredApplications" "FeatherMark" "Software\FeatherMark\Capabilities"
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'

  IfSilent feathermark_defaults_done
  MessageBox MB_YESNO|MB_ICONQUESTION \
    "Would you like to choose FeatherMark as the default app for .md and .markdown files?$\r$\n$\r$\nWindows Default Apps will open so you can confirm the choice." \
    IDNO feathermark_defaults_done
  ExecShell "open" "ms-settings:defaultapps?registeredAppUser=FeatherMark"

feathermark_defaults_done:
!macroend

!macro NSIS_HOOK_POSTUNINSTALL
  DeleteRegValue HKCU "Software\RegisteredApplications" "FeatherMark"
  DeleteRegKey HKCU "Software\FeatherMark"
  DeleteRegValue HKCU "Software\Classes\.md\OpenWithProgids" "FeatherMark.Markdown"
  DeleteRegValue HKCU "Software\Classes\.markdown\OpenWithProgids" "FeatherMark.Markdown"
  DeleteRegKey /ifempty HKCU "Software\Classes\.md\OpenWithProgids"
  DeleteRegKey /ifempty HKCU "Software\Classes\.markdown\OpenWithProgids"
  DeleteRegKey HKCU "Software\Classes\Applications\feathermark.exe"
  DeleteRegKey HKCU "Software\Classes\SystemFileAssociations\.md\shell\FeatherMark.Open"
  DeleteRegKey HKCU "Software\Classes\SystemFileAssociations\.markdown\shell\FeatherMark.Open"
  DeleteRegKey HKCU "Software\Classes\FeatherMark.Markdown"
  System::Call 'shell32::SHChangeNotify(i 0x08000000, i 0, p 0, p 0)'
!macroend
