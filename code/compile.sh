#!/bin/bash
count=0
> error.log
for d in */ ; do
    for z in $d*.rs ; do
	out=$((rustc $z) 2>&1)
	if echo "$out" | grep -q "error"; then
	    echo "$z failed to compile!"
	    echo "$z" >> error.log
	    echo "$out" >> error.log
	    echo "---------------------------------------------" >> error.log
	    count=$((count+1))
	else
	    if [ -f ${z:3:-3} ]; then
		rm ${z:3:-3}
	    fi
	fi
    done
done
if [ "$count" -eq 0 ] ; then
    echo "All files successfully compiled!"
    rm error.log
else
    echo "$count files failed to compile! (See error.log)"
fi
