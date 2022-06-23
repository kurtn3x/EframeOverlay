A Simple Overlay using the Eframe Template / Egui. Can be used for drawing overlays over games or other stuff. It's a transparent window that lets all input pass trough, so it doesnt block any input. Only allows input if hovering over buttons for example.

Simply a showcase, wanted to use this to build a bigger app but Rust is the wrong language for what i want.

The "main code" is located under /src. The rest are dependencies, that had to be used because release version didnt have the stuff i needed at the time.

Features:
- Hotkeys ( can capture hotkeys and does stuff when pressed )
- Edit Mode ( can change button position per drag&drop , text scale and other stuff )
