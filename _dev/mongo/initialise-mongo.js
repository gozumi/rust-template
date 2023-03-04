const adminDb = new Mongo().getDB('admin');
const { 
    MONGO_ADMIN_USER,
    MONGO_ADMIN_PASSWORD,
    MONGO_USER, 
    MONGO_PASSWORD
} = process.env

adminDb.createUser({
    user: MONGO_ADMIN_USER,
    pwd: MONGO_ADMIN_PASSWORD,
    roles: ['userAdminAnyDatabase', 'readWriteAnyDatabase']
})

const applicationDbName = 'APPLICATION';
const db = new Mongo().getDB(applicationDbName);
db.createUser({
    user: MONGO_USER,
    pwd: MONGO_PASSWORD,
    roles: [
        {
            db: applicationDbName,
            role: 'readWrite'
        }
    ]
})
