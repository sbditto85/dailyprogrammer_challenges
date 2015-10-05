import sys
#not completely done but meh gotta work
#https://www.reddit.com/r/dailyprogrammer/comments/3ltee2/20150921_challenge_233_easy_the_house_that_ascii/


file = sys.argv[1]

# with open(file, 'r') as f:
#     numlines = int(f.readline())

#     lines = []
#     for line in f:
#         lines.append(line.strip('\n'))

#     lines.reverse()

#     #handle bottom
#     bottom = lines[0]
#     lines = lines[1::]
    
#     lines.reverse()
#     for (idx, line) in enumerate(lines):
#         length = len(line)
#         for i, char in enumerate(line):
#             #line 1
#             if char == '*':
#                 #if the previous one is a ' ' then start with +
#                 #if next one is a ' ' then end with a +
#                 if i == 0 or (i > 0 and line[i-1] == ' '):
#                     sys.stdout.write('+---')
#                 else:
#                     sys.stdout.write('----')
#                 if i == length - 1 or (i < length and line[i+1] == ' '):
#                     sys.stdout.write('+')
#                 else:
#                     sys.stdout.write('-')
#             else:
#                 sys.stdout.write('     ')
#         print ""
#         for i, char in enumerate(line):
#             #line 2
#             if char == '*':
#                 #if the previous one is a ' ' then start with +
#                 #if next one is a ' ' then end with a +
#                 if i == 0 or (i > 0 and line[i-1] == ' '):
#                     sys.stdout.write('|   ')
#                 else:
#                     sys.stdout.write('    ')
#                 if i == length - 1 or (i < length and line[i+1] == ' '):
#                     sys.stdout.write('|')
#                 else:
#                     sys.stdout.write(' ')
#             else:
#                 sys.stdout.write('     ')
#         print ""
#         for i, char in enumerate(line):
#             #line 3
#             if char == '*':
#                 #if the previous one is a ' ' then start with +
#                 #if next one is a ' ' then end with a +
#                 if i == 0 or (i > 0 and line[i-1] == ' '):
#                     sys.stdout.write('+---')
#                 else:
#                     sys.stdout.write('----')
#                 if i == length - 1 or (i < length and line[i+1] == ' '):
#                     sys.stdout.write('+')
#                 else:
#                     sys.stdout.write('-')
#             else:
#                 sys.stdout.write('     ')
#         print ""

    
#     bottom = bottom[1:-1]
#     sys.stdout.write('+----')
#     for _ in bottom:
#         sys.stdout.write('-----')
#     print '----+'
     
#     sys.stdout.write('|    ')
#     for _ in bottom:
#         sys.stdout.write('     ')
#     print '    |'
    
#     sys.stdout.write('+----')
#     for _ in bottom:
#         sys.stdout.write('-----')
#     print '----+'

    

