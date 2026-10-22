# Using LDtk

We use LDtk (Level Designer Toolkit) to build our map.

## Installing LDtk

LDtk is available on their website, but version 1.5.3 has a bug that prevents
Aseprite files from loading. This has been fixed on the dev branch, but has not
yet been released.

I've gone ahead and created a
[macOS](https://github.com/jgayfer/ldtk/actions/runs/39694201967/artifacts/12389913629)
and
[Windows](https://github.com/jgayfer/ldtk/actions/runs/39694201967/artifacts/12389414195)
build from the unreleased dev branch. This version allows us to use our Aseprite
files directly in LDtk, removing the need to constantly export from Aseprite to
PNG.

Your computer may scream at you, as the executable is not signed. It's fine.
Trust me.

## Adding a tileset

Ensure your Aseprite tileset is saved under the `assets/` directory.

Tilesets can be imported into LDtk from the "Tilesets" menu (the icon is a paint
brush).

Depending on how your tileset is configured, you may need to adjust the "Tiles
layout" config when creating the tileset. In my experience Aseprite doesn't add
any padding or spacing (usually we only see this in exported PNGs), so the main
thing to check is the tile size (e.g. 16px).

If configured properly, the tiles should appear correctly when hovering over
them in the preview.

## Tile layers

Before we can start painting tiles, we need to create a tile layer with our
tileset.

Open the layers tab, and add a new "Tiles" layer. Give it a descriptive name,
and set your newly added tileset as the layer's default tileset.

If you're feeling brave, you can create an "Auto-layer". Auto-layers have smart
rules for painting, which are incredibly powerful. I haven't quite figured them
out yet.

We'll likely only have one tile layer for now, so feel free to replace the
tileset in the "map" layer with yours. This layer is already registered in Bevy,
so no code change is needed.

## Painting tiles

With our tile layer added, we can start painting it.

Select our newly added tile layer in the sidebar. This will open the tileset
picker on the bottom left. Pick which tile you'd like to place, and then place
it on the map.

## Update the game

Any changes to a layer that has been registered in the game will auto-reload.

Note that newly added layers will not show up in the game right away -- a code
change is required.


