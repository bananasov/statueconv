-- I stole some of this from sc-peripherals print3d.lua
-- Original: https://github.com/SwitchCraftCC/sc-peripherals/blob/1.20.1/src/main/resources/data/computercraft/lua/rom/programs/print3d.lua

local args = { ... }

local function printUsage()
    local programName = arg[0] or fs.getName(shell.getRunningProgram())
    print("Usage: ")
    print(programName .. " <file.statue>")
end

if #args < 1 or #args > 2 then
    printUsage()
    return
end

local filename = args[1]
if not fs.exists(filename) then printUsage() end

local f = fs.open(filename, "r")
local contents = f.readAll()
f.close()

local workbench = peripheral.find("statue_workbench")
local statue = textutils.unserializeJSON(contents)

local function less_stupid_to_stupid(cubes)
    local stupid = {}

    for _, cube in pairs(cubes) do
        stupid[#stupid+1] = {
            x1 = cube.bounds.x1,
            y1 = cube.bounds.y1,
            z1 = cube.bounds.z1,

            x2 = cube.bounds.x2,
            y2 = cube.bounds.y2,
            z2 = cube.bounds.z2,

            tint = cube.tint,
            texture = cube.texture
        }
    end

    return stupid
end

if workbench.isPresent() then
    if statue.name then workbench.setStatueName(statue.name) end
    if statue.author then workbench.setAuthor(statue.author) end
    if statue.light_level then workbench.setLightLevel(statue.light_level) end
    if statue.cubes then
    	workbench.setCubes(less_stupid_to_stupid(statue.cubes))
    end
    print("Printed", statue.name)
end