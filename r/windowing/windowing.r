
library(tcltk)

# Create a function to handle button click event
buttonClick <- function() {
  tkmessageBox(message = "Button clicked!")
}

# Create a window with larger dimensions
window <- tktoplevel()
tkwm.title(window, "Event Loop Example")
tkwm.geometry(window, "400x300")  # Set the window size to 400x300

# Create a label
label <- tklabel(window, text = "Click the button!")
tkpack(label)

# Create a button
button <- tkbutton(window, text = "Click Me!", command = buttonClick)
tkpack(button)

# Event loop
tkbind(window, "<Destroy>", function() tkdestroy(window))
tkwait.window(window)
