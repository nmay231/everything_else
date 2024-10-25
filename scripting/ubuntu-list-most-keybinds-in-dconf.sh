# To list (hopefully) all keybinds excluding custom keybinds (because those are located in multiple places) Ubuntu 22.04.

TOP_LEVEL_SETTINGS=$(gsettings list-recursively | awk '{ print $1 }' | grep key | sort | uniq)

for SETTING in $TOP_LEVEL_SETTINGS; do
	gsettings list-recursively $SETTING;
done

#cat <(gsettings list-recursively org.gnome.desktop.wm.keybindings) <(gsettings list-recursively org.gnome.settings-daemon.plugins.media-keys) <(gsettings list-recursively org.gnome.settings-daemon.plugins.power) | less
