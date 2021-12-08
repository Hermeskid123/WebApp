import sys
import mysql.connector
from mysql.connector import errorcode

if(len(sys.argv) < 3):
    exit(1)



file = open('data_base.txt', 'r')
l = ['','','','','']

count = -1
for line in file:
    count = count + 1
    l[count] = line.strip()
U = l[0]
P = l[2]
H= l[3]
DB = l[1]

try:
    cnx = mysql.connector.connect(user=U, password=P,host=H,database=DB)

except mysql.connector.Error as err:
  if err.errno == errorcode.ER_ACCESS_DENIED_ERROR:
    print("Something is wrong with your user name or password")
  elif err.errno == errorcode.ER_BAD_DB_ERROR:
    print("Database does not exist")
  else:
    print(err)
else:
    sql = 'INSERT INTO users VALUES (0,%s,%s,%s)'
    mycursor = cnx.cursor()    
    vals = sys.argv[1],sys.argv[2],sys.argv[3]
    mycursor.execute(sql,vals)
##    print(vals)
    cnx.commit()
    print("0")
    cnx.close()
