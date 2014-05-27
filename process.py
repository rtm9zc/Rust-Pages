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
        
def processPre(fileLoc, summary):
        preLoc = 'pre/' + fileLoc
        mdLoc = 'md/' + fileLoc
        mdLoc = mdLoc[:-4] + '.md'
        fileIn = open(preLoc, 'r')
        fileOut = open(mdLoc, 'w')
        for line in fileIn.readlines():
                if line[:2] == '# ':
                        fileOut.write(line)
                        summary.write('* [' + line[2:-1] + '](' + mdLoc + ')\n')
                elif line[:3] == '## ':
                        templine = line[3:-1]
                        link = templine.replace(' ', '_')
                        fileOut.write('<a name="' + link + '"></a>')
                        summary.write('    * [' + templine + 
                                      '](' + mdLoc + '#' + link + ')\n')
                elif line[:8] == '//inline':
                        params = line.split()
                        codeProc(params[1], params[2], fileOut)
                else:
                        fileOut.write(line);

summary = open('SUMMARY.md', 'w')
summary.write('# Summary\n\n')
files = os.listdir("pre")
files.sort()
for fileName in files:
        if fileName[-4:] == '.pre':
                processPre(fileName, summary)