extends Node

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		print("Mouse Motion at: ", event.position)
