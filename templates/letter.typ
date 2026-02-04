#import sys: inputs

#let red = rgb("#DD292C")
#let yellow = rgb("#F5C730")
#let blue = rgb("#244087")

#let boxpadding = (y: 8pt, x: 15pt)
#set text(font: "Poppins")
#set list(marker: "✿")

#let from = inputs.at("from", default: "Amadeus");
#let to = inputs.at("to", default: "Constanze");
#let msg = inputs.at("msg", default: lorem(120));

#align(center)[
  #image("logo.jpg", width: 20%)
]

#set text(size: 16pt)

Abs: *#from*

An: *#to*

#msg



