#!/bin/bash
# run this from the host environment
ramfs_size_mb=100
mount_point="./FuzzVolume"

mkramdisk() {
  ramfs_size_sectors=$((${ramfs_size_mb}*1024*1024/512))
  ramdisk_dev=`hdid -nomount ram://${ramfs_size_sectors}`

  newfs_hfs -v 'ram disk' ${ramdisk_dev}
  mkdir -p ${mount_point}
  mount -o noatime -t hfs ${ramdisk_dev} ${mount_point}

  echo "remove by re-running with the directory existing"
}

if [[ -d "./FuzzVolume" ]]; then
	echo "Removing" && \
	umount ${mount_point} && \
	diskutil eject "ram disk" && \
	rm -r ${mount_point}
else
	mkramdisk
fi
