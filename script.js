/*--------------------
Utils
--------------------*/
const scale = (v, x1, y1, x2, y2) => (v - x1) * (y2 - x2) / (y1 - x1) + x2;
const lerp = (a, b, t) => a + t * (b - a)

/*--------------------
Settings
--------------------*/
const settings = {
    backgroundColor: '#fff',
    letters: '啊啊啊，我要给你生猴子，不要跑！',
    minDistance: 14,
    resetTimer: 2000,
}

/*--------------------
Setup
--------------------*/
const canvas = document.getElementById('canvas')
const ctx = canvas.getContext('2d')
const win = {
    w: window.innerWidth,
    h: window.innerHeight
}
const mouse = {
    x: win.w / 2,
    y: win.h / 2,
}
let time = 0
const devicePixelRatio = window.devicePixelRatio || 1
const chain = []
const letters = settings.letters.split('').reverse()
for (let i = 0; i < letters.length; i++) {
    chain.push({ letter: letters[i], x: 0, y: 0 })
}


/*--------------------
Resize
--------------------*/
const onResize = () => {
    win.w = window.innerWidth
    win.h = window.innerHeight
    canvas.width = win.w * devicePixelRatio
    canvas.height = win.h * devicePixelRatio
    canvas.style.width = `${win.w}px`
    canvas.style.height = `${win.h}px`
    ctx.scale(devicePixelRatio, devicePixelRatio)
}
onResize()


/*--------------------
MouseMove
--------------------*/
let isMouseMoving = false
let timeoutID
const onMouseMove = (e) => {
    isMouseMoving = true
    mouse.x = e.touches ? e.touches[0].clientX : e.clientX
    mouse.y = e.touches ? e.touches[0].clientY : e.clientY

    chain[0].x = mouse.x
    chain[0].y = mouse.y

    clearTimeout(timeoutID)
    timeoutID = setTimeout(() => {
        isMouseMoving = false
    }, settings.resetTimer)
}


/*--------------------
Listeners
--------------------*/
window.addEventListener('resize', onResize)
window.addEventListener('mousemove', onMouseMove)
window.addEventListener('touchmove', onMouseMove)


/*--------------------
Clear
--------------------*/
const clear = () => {
    ctx.clearRect(0, 0, canvas.width, canvas.height)
}


/*--------------------
Draw
--------------------*/
const draw = () => {
    clear()

    chain.forEach((link, index) => {
        if (isMouseMoving) {
            ctx.fillText(link.letter, link.x - settings.minDistance, link.y)

            if (index > 0) {
                let prevLink = chain[index - 1]
                let dx = link.x - prevLink.x
                let dy = link.y - prevLink.y
                let distance = Math.sqrt(dx * dx + dy * dy)

                if (distance > settings.minDistance) {
                    let ratio = settings.minDistance / distance
                    link.x = lerp(link.x, prevLink.x + dx * ratio, .4)
                    link.y = lerp(link.y, prevLink.y + dy * ratio, .4)
                }
            }
        } else {
            const theta = scale(index, 0, chain.length, .3, .06)

            link.x = lerp(link.x, mouse.x - (index + 1) * settings.minDistance, theta)
            link.y = lerp(link.y, mouse.y + Math.sin(time * .3 + index * .5) * 3, theta)
            ctx.fillText(link.letter, link.x, link.y)
        }
    })
}


/*--------------------
Animate
--------------------*/
const animate = () => {
    time += 0.1
    clear()
    requestAnimationFrame(animate)

    draw()
}
animate()