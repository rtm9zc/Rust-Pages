import os

def codeProc(rustLoc, blockNum, fileOut):
        rust = 'code/' + rustLoc
        rustFile = open(rust)
        fileOut.write("```rust\n")
        writeflag = False
        for line in rustFile.readlines():
                if writeflag:
                        if line[:5] == '//end':
                                numcheck = line.split()[1]
                                if numcheck == blockNum:
                                        break
			elif line[:8] == '//inline':
					continue
                        else:
                                fileOut.write(line)
                else:
                        if line[:8] == '//inline':
                                numcheck = line.split()[1]
                                if numcheck == blockNum:
                                        writeflag = True

        fileOut.write("```\n");
        
def codeBloc(rustLoc, blockNum, fileOut): #used for example blocks
        rust = 'code/' + rustLoc
        rustFile = open(rust)
        fileOut.write(">```rust\n")
        writeflag = False
        for line in rustFile.readlines():
                if writeflag:
                        if line[:5] == '//end':
                                numcheck = line.split()[1]
                                if numcheck == blockNum:
                                        break
			elif line[:8] == '//inline':
					continue
                        else:
                                fileOut.write('>' + line)
                else:
                        if line[:8] == '//inline':
                                numcheck = line.split()[1]
                                if numcheck == blockNum:
                                        writeflag = True

        fileOut.write("```\n");
        
def processPre(fileLoc, summary):
        preLoc = 'pre/' + fileLoc
        mdLoc = 'md/' + fileLoc
        mdLoc = mdLoc[:-4] + '.md'
        fileIn = open(preLoc, 'r')
        fileOut = open(mdLoc, 'w')
	pastIntro = False
	headct = 1
	subLoc = mdLoc[:-3]
	if not os.path.exists(subLoc):
		os.makedirs(subLoc)
        for line in fileIn.readlines():
                if line[:2] == '# ':
                        fileOut.write(line)
                        summary.write('* [' + line[2:-1] + '](' + mdLoc + ')\n')
                elif line[:3] == '## ':
			if not pastIntro:
				pastIntro = True
				subLoc = subLoc + '/' + `headct` + '.md'
			else:
				subFile.close()
				subLoc = mdLoc[:-3] + '/' + `headct` + '.md'
			subFile = open(subLoc, 'w')
			headct += 1
                        linkName = line[3:-1]
                        fileOut.write(line)
			subFile.write(line)
                        summary.write('    * [' + linkName + '](' + subLoc + ')\n')
                elif line[:8] == '//inline':
                        params = line.split()
                        codeProc(params[1], params[2], fileOut)
			if pastIntro:
				codeProc(params[1], params[2], subFile)
		elif line[:9] == '>//inline':
                        params = line.split()
                        codeBloc(params[1], params[2], fileOut)
			if pastIntro:
				codeBloc(params[1], params[2], subFile)
                else:
                        fileOut.write(line);
			if pastIntro:
				subFile.write(line)

summary = open('SUMMARY.md', 'w')
summary.write('# Summary\n\n')
files = os.listdir("pre")
files.sort()
for fileName in files:
        if fileName[-4:] == '.pre':
                processPre(fileName, summary)
