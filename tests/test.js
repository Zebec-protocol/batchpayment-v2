//client side test code for testing batch-payment

const BufferLayout = require("buffer-layout");
const sol = require("@solana/web3.js"); 
const spl = require("@solana/spl-token");
const borsh =require("borsh")
const cluster = sol.clusterApiUrl("devnet", true);

console.log(cluster);

const programAddr = "HQC8Ug9qbmKwbb5L5tcVMwXC3unUonSnMBg9PgfcToDu"
let prm=new sol.PublicKey(programAddr);
let alice = sol.Keypair.fromSecretKey(Buffer.from(
    [
        60,148,236,18,180,159,40,102,143,19,204,248,55,249,10,73,134,20,163,148,244,73,46,249,112,167,163,1,229,230,14,69,162,130,19,80,226,16,27,238,16,222,192,67,219,179,45,83,57,57,142,9,160,60,229,83,20,181,175,120,61,129,155,85]
        ));
        
let pda_data =new sol.Keypair(); // for cpi
let receiver1=new sol.Keypair(); 
let receiver2=new sol.Keypair();
let receiver3=new sol.Keypair();
class Setting {
    constructor(args) {
        this.instruction = 0;
        this.percent = [800000,100000,100000];//1000000
    }
}

const Set = new Map([
    [
        Setting,
        {
            kind: "struct",
            fields: [
                ["instruction", "u8"],
                ["percent", ["u64"]],
            ],
        },
    ],
]);


const depositLayout = BufferLayout.struct([
    BufferLayout.u8("instruction"),
    BufferLayout.blob(8, "amount"),
  
]);
const claimLayout = BufferLayout.struct([
    BufferLayout.u8("instruction"),
  
]);
///
async function set(connection) {

    const val = new Setting();
    const data = borsh.serialize(Set, val);
    
    
    console.log("ADMIN: %s", alice.publicKey.toBase58()); //admin              
    console.log("DATA:   %s", pda_data.publicKey.toBase58()); // Data storage PDA
    console.log("Receiver1:   %s", receiver1.publicKey.toBase58()); 
    console.log("Receiver2:   %s", receiver2.publicKey.toBase58()); 
    console.log("Receiver3:   %s", receiver3.publicKey.toBase58()); 
    const [vault,_bonce]=await sol.PublicKey.findProgramAddress(
        [alice.publicKey.toBuffer(),Buffer.from("batchv2"),],
          prm,
        );
    const instruction = new sol.TransactionInstruction({
        keys: [
            {
                //initializer
                pubkey: alice.publicKey,
                isSigner: true,
                isWritable: true,
            }, 
           {
                //system program
                pubkey:  sol.SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },  
            {
                //vault
                pubkey: vault,
                isSigner: false,
                isWritable: true,
            }, {
                //data saved 
                pubkey: pda_data.publicKey,
                isSigner: true,
                isWritable: true,
            }, 
            {
                //to be called
                pubkey: receiver1.publicKey,
                isSigner: false,
                isWritable: true,
            }, 
            {
                //to be called
                pubkey: receiver2.publicKey,
                isSigner: false,
                isWritable: true,
            }, 
            {
                //to be called
                pubkey: receiver3.publicKey,
                isSigner: false,
                isWritable: true,
            }, 

    ],
        programId: prm,
        data: data,
    });
    // Transaction signed by 
    tx = new sol.Transaction().add(instruction);
    return await sol.sendAndConfirmTransaction(connection, tx, [alice,pda_data],
        );
}
async function claim(connection) {  
    var data = Buffer.alloc(claimLayout.span);
    claimLayout.encode({
            
            instruction: 1,
        },
        data,
    ); 
    const [vault,_bonce]=await sol.PublicKey.findProgramAddress(
        [alice.publicKey.toBuffer(),Buffer.from("batchv2"),],
          prm,
        );
    const instruction = new sol.TransactionInstruction({
        keys: [
            {
                //receiver
                pubkey:  "GhcKA56tRv6tSAbUUDJhtPTXPRxr977LKJeLw5SVNCR9",
                isSigner: false,
                isWritable: true,
            }, 
            {
                //payer // Can be anyone //for testing purpose only must be signer
                pubkey: alice.publicKey,
                isSigner: true,
                isWritable: true,
            }, 
            {
                pubkey:  "31G3RAEqimHEMjEBxHKfVVrLMr5vzbwbW53Gs23859os",
                isSigner: false,
                isWritable: true,
            }, 
            {
                pubkey:  vault,
                isSigner: false,
                isWritable: true,
            }, 
        {
            // This is the system program public key.
            pubkey: sol.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },

           

    ],
        programId: prm,
        data: data,
    });
                
  
    
    // Transaction signed by 
    tx = new sol.Transaction().add(instruction);

    return await sol.sendAndConfirmTransaction(connection, tx, [alice],);
}
async function deposit(connection) {  
    var data = Buffer.alloc(depositLayout.span);
    depositLayout.encode({
            
            instruction: 2,
            amount: new spl.u64(sol.LAMPORTS_PER_SOL).toBuffer(),
        },
        data,
    ); 
    const [vault,_bonce]=await sol.PublicKey.findProgramAddress(
        [alice.publicKey.toBuffer(),Buffer.from("batchv2"),],
          prm,
        );
    const instruction = new sol.TransactionInstruction({
        keys: [
            {
                //depositer_can_be anyone
                pubkey:  alice.publicKey,
                isSigner: true,
                isWritable: true,
            }, 
            {
                //send to find out the vault no need to be signer in this case only
                pubkey: alice.publicKey,
                isSigner: true,
                isWritable: true,
            },  
        {
            // This is the system program public key.
            pubkey: sol.SystemProgram.programId,
            isSigner: false,
            isWritable: false,
        },
        //vault
            {
            pubkey:  vault,
            isSigner: false,
            isWritable: true,
        },

    ],
        programId: prm,
        data: data,
    });
                
  
    
    // Transaction signed by 
    tx = new sol.Transaction().add(instruction);

    return await sol.sendAndConfirmTransaction(connection, tx, [alice],);
}


async function main(args) {



    const conn = new sol.Connection(cluster);
    switch (args[2]) {
         case "s":
            console.log("TXID:", await set(conn));
            break;
        case "p":
            
            console.log("TXID:", await deposit(conn));
            break;
        case "c":
        
            console.log("TXID:", await claim(conn));
            break;
    default:
        break;
    }
}

main(process.argv).then(() => process.exit(0)).catch(e => console.error(e));