# Texture Map

The texture map is 256x256 pixels, so there are 16x16 items in the texture map.

## Items

Items consist of a 16x16 subimage of the texture map.  Some items, such as apparel, have multiple subimages for profiles and animations.  Animations are generally used for legs or items used for legs.  Animated images contain 4 subimages/profiles.  The first is front, the second is side, the third and fourth are animated sides.

## Tiles

Tiles are 32x32 pixels that are enumerated onto a generated map

## Characters and Mobs

Characters and mobs are 32x32 pixels per profile.  There are 5 profiles per character or mob.  The first profile is the front-facing profile, the second is the back-facing profile, the third is the side-facing.  The last 2 are animated sides.  These are preferably the right side, since the code is right side oriented.  All sides will be flipped for the left side.