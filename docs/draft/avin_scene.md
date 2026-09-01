Самый важный conceptual split
Вот его стоит сейчас просто запомнить:

Scene отвечает:
Что рисовать и где?
Например:
эта свеча находится в world coordinate:
x = timestamp
y = price

Viewport отвечает:
Какую часть мира сейчас видит пользователь?
x: 10:00 → 14:00
y: 280 → 320

Transform отвечает:
Как world coordinate превратить в screen coordinate?
Например:
price 300 → y=417 px
timestamp 12:00 → x=822 px

Renderer отвечает:
Нарисуй мне линию от (822, 417) до (822, 460).
И ему уже похуй, что это цена, свеча, биржа или AVIN.

GUI	Самый естественный avin_scene
Iced	        wgpu
Dioxus Web	    wgpu → WebGPU → canvas
GTK4	        OpenGL → GtkGLArea  || GSK
