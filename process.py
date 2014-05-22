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
                        else:
                                fileOut.write(line)
                else:
                        if line[:8] == '//inline':
                                numcheck = line.split()[1]
                                if numcheck == blockNum:
                                        writeflag = True

        fileOut.write("```\n");
        
def processPre(fileLoc):
        preLoc = 'pre/' + fileLoc
        mdLoc = 'md/' + fileLoc
        mdLoc = mdLoc[:-4] + '.md'
        fileIn = open(preLoc, 'r')
        fileOut = open(mdLoc, 'w')
        for line in fileIn.readlines():
                if line[:7] == '//label':
                        templine = line[7:]
                        fileOut.write('<a name="' + templine + '"></a>')
                elif line[:8] == '//inline':
                        params = line.split()
                        codeProc(params[1], params[2], fileOut)
                else:
                        fileOut.write(line);


files = os.listdir("pre")
for fileName in files:
        if fileName[-4:] == '.pre':
                processPre(fileName)
