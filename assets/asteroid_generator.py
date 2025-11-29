import bpy
import random
import os

### NOTICE ###
# This script is a blender script use to generate asteroid like meshes using random displacement
# Running the script by itself wont result in any mesh

# User config
u_subdivision = 2
u_scale = 1
u_strength = 0.80

# Clear scene
bpy.ops.object.select_all(action="DESELECT")
bpy.ops.object.select_by_type(type="MESH")
bpy.ops.object.delete()

# Create mesh
bpy.ops.mesh.primitive_ico_sphere_add(
    subdivisions=u_subdivision,
    radius=1.0,
    enter_editmode=False,
    align="WORLD",
    location=(0, 0, 0),
    scale=(u_scale, u_scale, u_scale),
)

# Create displace modifier & noise texture
obj = bpy.context.active_object

modifier = obj.modifiers.new(name="Displace", type="DISPLACE")

texture = bpy.data.textures.new(name="DisplaceTexture", type="DISTORTED_NOISE")

modifier.texture = texture

modifier.strength = u_strength

bpy.ops.object.empty_add(type="PLAIN_AXES")
empty = bpy.context.active_object
empty.name = "TextureControl"

modifier.texture_coords = "OBJECT"
modifier.texture_coords_object = empty

# Randomize noise
empty.location.x = random.uniform(-100, 100)
empty.location.y = random.uniform(-100, 100)
empty.location.z = random.uniform(-100, 100)

bpy.context.view_layer.objects.active = obj

# Apply modifier
bpy.ops.object.modifier_apply(modifier=modifier.name)

bpy.data.objects.remove(empty, do_unlink=True)

# Export glb
bpy.context.view_layer.objects.active = obj
obj.select_set(True)

base_path = "assets/asteroids/XS"
file_path = f"{base_path}0.glb"
i = 1
while os.path.exists(file_path):
    file_path = f"{base_path}{i}.glb"
    i += 1

bpy.ops.export_scene.gltf(filepath=file_path, use_selection=True)
