# Why Gentoo is the best distribution?

Because you have the ultimate control, you can do everything you wish.

## Patching source code

In Gentoo if you don't like some software you can apply a patch. Read more about patches 
on the Gentoo Wiki: https://wiki.gentoo.org/wiki//etc/portage/patches

For example I use the following patch on `app-misc/fastfetch`
```
diff --git a/src/logo/ascii/gentoo.txt b/src/logo/ascii/gentoo.txt
index a90dd331..3b5fb5f1 100644
--- a/src/logo/ascii/gentoo.txt
+++ b/src/logo/ascii/gentoo.txt
@@ -3,7 +3,7 @@
    -y$2NMMMMMMMMMMMNNNmmdhy$1+-
  `o$2mMMMMMMMMMMMMNmdmmmmddhhy$1/`
  om$2MMMMMMMMMMMN$1hhyyyo$2hmdddhhhd$1o`
-.y$2dMMMMMMMMMMd$1hs++so/s$2mdddhhhhdm$1+`
+.y$2dMMMMMMMMMMd$1hs  so/s$2mdddhhhhdm$1+`
  oy$2hdmNMMMMMMMN$1dyooy$2dmddddhhhhyhN$1d.
   :o$2yhhdNNMMMMMMMNNNmmdddhhhhhyym$1Mh
     .:$2+sydNMMMMMNNNmmmdddhhhhhhmM$1my
```

## Debloating your system

Using `equery` from `app-portage/gentoolkit` you can check for exact description of USE flags. 
With this knowledge you can disable any undesired features.
