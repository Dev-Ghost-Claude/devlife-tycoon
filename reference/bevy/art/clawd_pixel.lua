-- Draws pixel Clawd's jump cycle into art/clawd_pixel.aseprite.
-- Frames are anchored at the feet; Bevy moves the sprite up and down.
local out = app.params.out
local W, H = 24, 24

local pc = app.pixelColor
local BODY  = pc.rgba(0xD9, 0x77, 0x57, 255)  -- Claude terracotta
local LIGHT = pc.rgba(0xE8, 0x95, 0x76, 255)  -- top edge, light from above
local SHADE = pc.rgba(0xB4, 0x58, 0x3B, 255)  -- bottom edge and legs
local EYE   = pc.rgba(0x2A, 0x18, 0x12, 255)  -- warm near-black

local function rect(img, x0, y0, x1, y1, c)
  for y = y0, y1 do for x = x0, x1 do img:drawPixel(x, y, c) end end
end

-- bw/bh: body size; leg: leg length; eyes: "open" or "happy"; arm: arm lift.
local function clawd(bw, bh, leg, eyes, arm)
  local img = Image(W, H, ColorMode.RGB)
  local ground = H - 1
  local left = 12 - bw // 2
  local right = left + bw - 1
  local bottom = ground - leg
  local top = bottom - bh + 1

  -- legs: four thin legs, the outer pair one pixel in from the edge
  for _, x in ipairs({ left + 1, left + 3, right - 3, right - 1 }) do
    rect(img, x, bottom + 1, x, ground, SHADE)
  end
  -- body with a light top row and a shaded bottom row
  rect(img, left, top, right, bottom, BODY)
  rect(img, left, top, right, top, LIGHT)
  rect(img, left, bottom, right, bottom, SHADE)
  -- arms: little side nubs, lifted when happy
  local ay = top + bh // 2 - arm
  rect(img, left - 2, ay, left - 1, ay + 1, BODY)
  rect(img, right + 1, ay, right + 2, ay + 1, BODY)
  -- eyes
  local ex1, ex2, ey = left + 3, right - 3, top + 2
  if eyes == "happy" then
    for _, ex in ipairs({ ex1, ex2 }) do
      img:drawPixel(ex - 1, ey + 1, EYE)
      img:drawPixel(ex, ey, EYE)
      img:drawPixel(ex + 1, ey + 1, EYE)
    end
  else
    rect(img, ex1, ey, ex1, ey + 1, EYE)
    rect(img, ex2, ey, ex2, ey + 1, EYE)
  end
  return img
end

-- name, body w, body h, legs, eyes, arm lift, seconds
local frames = {
  { "idle",    14, 8,  2, "open",  0, 0.30 },
  { "crouch",  16, 6,  1, "open",  0, 0.12 },
  { "launch",  12, 10, 3, "happy", 2, 0.10 },
  { "rise",    14, 9,  2, "happy", 3, 0.16 },
  { "apex",    14, 8,  1, "happy", 3, 0.20 },
  { "fall",    13, 9,  3, "open",  1, 0.16 },
  { "land",    16, 6,  1, "happy", 0, 0.12 },
  { "settle",  14, 8,  2, "happy", 0, 0.14 },
}

local spr = Sprite(W, H, ColorMode.RGB)
for i, f in ipairs(frames) do
  if i > 1 then spr:newEmptyFrame() end
  spr:newCel(spr.layers[1], i, clawd(f[2], f[3], f[4], f[5], f[6]), Point(0, 0))
  spr.frames[i].duration = f[7]
end
spr.layers[1].name = "clawd"
spr:newTag(1, #frames).name = "jump"
spr:saveAs(out)
print("saved " .. out .. " with " .. #frames .. " frames")
