-- Proof piece: a 1978 home computer as a sprite stack (1 voxel = 1 pixel), rendered
-- spinning, to show hand-placed pixel detail surviving a turning camera.
local out = app.params.out
local FRAMES = tonumber(app.params.frames or 16)
local FW, FH = 56, 52            -- frame size
local W, D, H = 28, 22, 22       -- model size: x (width), y (depth, 0 = back), z (height)
local pc = app.pixelColor
local function hex(h) return pc.rgba(tonumber(h:sub(1,2),16), tonumber(h:sub(3,4),16), tonumber(h:sub(5,6),16), 255) end

local C = {
  beige = {0xE6,0xDA,0xBE}, beigeTop = {0xF2,0xE8,0xD2}, beigeShade = {0xC4,0xB6,0x96},
  dark = {0x5C,0x50,0x44}, key = {0xF4,0xEC,0xDA}, gap = {0x9E,0x92,0x7A},
  bezel = {0xD8,0xCC,0xB0}, screen = {0x1E,0x34,0x28}, text = {0x86,0xE0,0x8E},
  led = {0xE0,0x4A,0x3A}, vent = {0x7A,0x6E,0x5E}, stripe = {0xC8,0x6A,0x3E},
}

-- The model: returns a colour name for voxel (x, y, z), or nil for air.
local function voxel(x, y, z)
  -- Base unit with a sloped keyboard at the front.
  if x >= 1 and x <= 26 and y >= 6 and y <= 21 and z >= 0 then
    local top = 5
    if y >= 13 then top = 5 - math.floor((y - 13) / 3) end
    if z <= top then
      if z == top and y >= 13 and y <= 20 and x >= 3 and x <= 24 then
        -- Key caps on a 2-voxel grid, offset per row like a real keyboard.
        local row = y - 13
        local kx = x - 3 + (row % 2)
        if kx % 3 == 2 then return "gap" end
        return "key"
      end
      if y == 21 and z == 1 and x >= 3 and x <= 24 then return "stripe" end
      if y == 21 and z == 2 and x == 23 then return "led" end
      if x == 26 and z >= 1 and z <= 3 and y % 2 == 0 then return "vent" end
      return "beige"
    end
  end
  -- Monitor on the back of the base: a box with a CRT bulge behind it.
  local mz0, mz1 = 6, 19
  if z >= mz0 and z <= mz1 then
    local inset = 0
    if y < 7 then inset = 2 end
    if y < 4 then inset = 4 end
    if y >= 1 and y <= 13 and x >= 5 + inset and x <= 22 - inset then
      if y == 13 then
        -- The front face: bezel around a green-phosphor screen with lines of text.
        if x >= 8 and x <= 19 and z >= 8 and z <= 17 then
          -- Three lines of text and a cursor, read top to bottom (z counts upward).
          local lines = { [16] = 9, [14] = 6, [12] = 8 }
          local len = lines[z]
          if len and x >= 9 and x - 9 < len then return "text" end
          if z == 10 and (x == 9 or x == 10) then return "text" end
          return "screen"
        end
        return "bezel"
      end
      if x == 22 - inset and z >= 12 and z <= 16 and y % 2 == 0 then return "vent" end
      return "beige"
    end
  end
  return nil
end

-- Light comes from the upper left of the screen. Tops are brightest; side faces get
-- lighter or darker by which way they face after the model turns.
local LX, LY = -0.7, 0.7
local function shade(name, x, y, z, ca, sa)
  local c = C[name]
  if name == "beige" or name == "bezel" then
    if voxel(x, y, z + 1) == nil then c = C.beigeTop end
  end
  local f = 1.0
  if voxel(x, y, z + 1) ~= nil then
    -- A side voxel: find an open neighbour to know which way the face points.
    local nx, ny = 0, 0
    if voxel(x - 1, y, z) == nil then nx = -1 elseif voxel(x + 1, y, z) == nil then nx = 1
    elseif voxel(x, y + 1, z) == nil then ny = 1 elseif voxel(x, y - 1, z) == nil then ny = -1 end
    local rnx = nx * ca - ny * sa
    local rny = nx * sa + ny * ca
    local d = rnx * LX + rny * LY
    if d > 0.35 then f = 0.97 elseif d > -0.35 then f = 0.86 else f = 0.74 end
  end
  return { math.floor(c[1] * f), math.floor(c[2] * f), math.floor(c[3] * f) }
end

local spr = Sprite(FW, FH, ColorMode.RGB)
local cx, cy = FW / 2, FH / 2 + 10
for f = 1, FRAMES do
  if f > 1 then spr:newEmptyFrame() end
  local img = Image(FW, FH, ColorMode.RGB)
  local a = (f - 1) / FRAMES * 2 * math.pi + math.pi / 4
  local ca, sa = math.cos(a), math.sin(a)
  for z = 0, H do
    for sy = 0, FH - 1 do
      for sx = 0, FW - 1 do
        local rx = sx - cx + 0.5
        for sub = 0, 1 do
          local ry = (sy - cy + z) * 2 + sub
          local X = rx * ca + ry * sa
          local Y = -rx * sa + ry * ca
          local x = math.floor(X + W / 2)
          local y = math.floor(Y + D / 2)
          local v = voxel(x, y, z)
          if v then
            local c = shade(v, x, y, z, ca, sa)
            img:drawPixel(sx, sy, pc.rgba(c[1], c[2], c[3], 255))
            break
          end
        end
      end
    end
  end
  -- Selective outline: empty pixels touching the model get a darker version of the
  -- neighbour they touch, so the silhouette reads on any background.
  local outline = Image(FW, FH, ColorMode.RGB)
  outline:drawImage(img, Point(0, 0))
  for sy = 0, FH - 1 do
    for sx = 0, FW - 1 do
      if pc.rgbaA(img:getPixel(sx, sy)) == 0 then
        for _, d in ipairs({{1,0},{-1,0},{0,1},{0,-1}}) do
          local nx, ny = sx + d[1], sy + d[2]
          if nx >= 0 and ny >= 0 and nx < FW and ny < FH then
            local n = img:getPixel(nx, ny)
            if pc.rgbaA(n) > 0 then
              outline:drawPixel(sx, sy, pc.rgba(math.floor(pc.rgbaR(n) * 0.38), math.floor(pc.rgbaG(n) * 0.32), math.floor(pc.rgbaB(n) * 0.30), 255))
              break
            end
          end
        end
      end
    end
  end
  spr:newCel(spr.layers[1], f, outline, Point(0, 0))
  spr.frames[f].duration = 0.09
end
spr:newTag(1, FRAMES).name = "spin"
spr:saveAs(out)
print("saved " .. out)
