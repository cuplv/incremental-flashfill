#Reference: http://research.microsoft.com/en-us/um/people/sumitg/pubs/popl11-synthesis.pdf
import re
import itertools

class ConstStr:
    def __init__(self, s):
	self.s = s

class CPos:
    def __init__(self, p):
	self.p = p

class Pos:
    def __init__(self, r1, r2, c):
	self.r1 = r1
	self.r2 = r2
	self.c = c

class Substr:
    def __init__(self, v_i, y1, y2):
	self.v_i = v_i
	self.y1 = y1
	self.y2 = y2

def TokenSequence(s):
    TS = ""
    for char in s:
	if char >= 'A' and char <= 'Z':
	    TS += "U"
	elif char >= 'a' and char <= 'z':
	    TS += "L"
	elif char == ' ':
	    TS += "S"
    if len(s) == 0:
	return "E"
    return 'TS('+ ''.join(ch for ch, _ in itertools.groupby(TS)) + ')'

def ConcatenateTokenSequence(a, b):
    return "TS(" + a.split("TS(")[1].split(")")[0] + b.split("TS(")[1].split(")")[0] + ")"

def MatchTokenSequence(T, s):
    regex = ''

    if 'TS' in T:
	T = T.split("TS(")[1].split(")")[0]
    else:
	return regex

    for token in T:
	if token == 'U':
	    regex += '[A-Z]+'
        elif token == 'L':
	    regex += '[a-z]+'
	elif token == 'S':
	    regex += '\ +'
    return regex

def IParts(T, s):
    return T

def generateRegex(T, s):
    if T == "E":
	return ["E"]
    else:
        tokens = T.split("TS(")[1].split(")")[0] 
	regex = ""
	for token in tokens:
	    regex += IParts(token, s)
	regex = "TS(" + regex + ")"
        return [regex]

def generatePosition(s, k):
    #s - Input column
    #k - Start index of substring in the input column
    print "8) In generatePosition for", s, "and pos", k
 
    result = set([CPos(k), CPos(-(len(s) - k))])
	
    if k == 0:
	print "9)", "''", TokenSequence('')

    r1 = list() 
    k1_seq = list()
    for k1 in range(0, k):
	string = s[k1:k]
	token_sequence = TokenSequence(string)
	print "9)", string, token_sequence
        r1.append(token_sequence)
	k1_seq.append(k1)
    print

    r2 = list()
    k2_seq = list()
    for k2 in range(k+1, len(s)+1):
	string = s[k:k2]
        token_sequence = TokenSequence(string)
	print "10)", string, token_sequence
	r2.append(token_sequence)
	k2_seq.append(k2)
    print

    for a in r1:
	for b in r2:
	    r12 = ConcatenateTokenSequence(a, b)
            print "11) r12=", r12
	    regex = MatchTokenSequence(r12, s)
	
	    matches = list()
            for i in range(0, len(s)+1):
		for j in range(i+1, len(s)+1):
		    substring = s[i:j+1]
		    match = re.findall(regex, substring)
		    if len(match) == 1:
		         matches.append(match[0])
	    matches = list(set(matches)) 
 	    print "12) Matches", matches
 
            c_dash = len(matches)
	    print "13) c' = ", c_dash

	    k1 = k1_seq[r1.index(a)]
	    k2 = k2_seq[r2.index(b)] - 1

	    substring = s[k1:k2+1]
	    print "14) s[k1:k2]:", substring
     
            c = matches.index(substring)
	    print "15) c = ", c

            r1_dash = generateRegex(a, s)
	    r2_dash = generateRegex(b, s)
     
            print "16) r1_dash: ", r1_dash
	    print "17) r2_dash: ", r2_dash
        
	    positions = list()
            for reg1 in r1_dash:
		for reg2 in r2_dash:
		    for regc in [c, -(c_dash - c + 1)]:
                        p = Pos(reg1, reg2, regc)
			positions.append(p)

	    result = result.union(set(positions))
            print "result", list(result)
	    print
    return result
        
def generateSubstring(sigma, s):
    #sigma - List of input columns
    #s - substring
    print "5) In generateSubstring for", s
    result = set()

    matches = list()
    for i in range(0, len(sigma)):
        for match in re.finditer(re.escape(s), sigma[i]):
            matches.append((i, match.start()))
   
    for i,k in matches:
         print "6) Matches found in column", i, '=', sigma[i]
         print "7) Match: At pos", k, '=', str(sigma[i][k:len(s)+k])
         print

         y1 = generatePosition(sigma[i], k)
	 y2 = generatePosition(sigma[i], k + len(s))
 
         substrings = list()
         for reg_y1 in y1:
	     for reg_y2 in y2:
	         substrings.append(Substr(sigma[i], reg_y1, reg_y2))

         result = result.union(set(substrings))
    return result
   
def generateStr(sigma, s):
    #sigma - List of input columns
    #s - Output string
    nodes = set([i for i in xrange(len(s))])
    source_node = set([0])
    target_node = set([len(s)])

    edges = []
    for j in range(1, len(s)+1):
        for i in range(0, j):
	   edges.append((i, j)) 
    print "3) Edges:", edges
    print

    W = {}
    for edge in edges:
        i = edge[0]
        j = edge[1]
        substring = s[i:j]
        print "4) Calling generateSubstring on", substring
        W[(i,j)] = set([substring]).union(generateSubstring(sigma, substring))
	print "16) W: ", W 
    
    generateLoop(sigma, s, W)            
    #W = {edge:set([ConstStr(s[edge[0]:edge[1]-1])]).union(set(generateSubstring(sigma, s[edge[0]:edge[1]-1]))) for edge in edges}

def generateLoop(sigma, s, W):
    W_dash = W

    e1_dash = list()
    e2_dash = list()

    for k1 in range(0, len(s)+1):
        for k2 in range(0, len(s)+1):
            for k3 in range(0, len(s)+1):
                e1 = generateStr(sigma, s[k1:k2+1])
                print "17) e1 =", e1

def generateStringProgram(S):
    # S - Set of (sigma, s) pairs for input, output examples
    T = set()
    for each_input, each_output in S:
      each_sigma = each_input.split(',')
      print "1) Input:", each_sigma
      print "2) Output:", each_output
      print
      generateStr(each_sigma, each_output)

S = set([("Foo Bar","FB"), ("Text, String", "T")])
generateStringProgram(S)
