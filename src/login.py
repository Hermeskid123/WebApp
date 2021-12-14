import sys
import mysql.connector
from mysql.connector import errorcode

if(len(sys.argv) < 2):
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
    #sql = 'SELECT id ,username FROM users WHERE username = "P" and password = "hash"'
    sql = 'SELECT * FROM `users` WHERE 1'
    mycursor = cnx.cursor()    
    vals = sys.argv[1],sys.argv[2]
    mycursor.execute(sql)
    myresult = mycursor.fetchall()
    cnx.commit()
    for x in myresult:
        if( x[1] == sys.argv[1] and x[3] == sys.argv[2]):
            file = open("login.txt","w")
  #          file.write(str(x[0])) 
    #        file.write('\n') 
            file.write(str(x[1]))
            print(str(x[1]))
           # file.write('\n') 
#            file.write(str(x[2])) 
            file.close()
    cnx.close()

