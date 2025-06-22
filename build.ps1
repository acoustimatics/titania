param($Name)
ml64.exe /c /Fo "${Name}.obj" "${Name}.asm"
link.exe /subsystem:console /entry:"${Name}" "${Name}.obj"
