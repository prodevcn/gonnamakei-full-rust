<template>
    <div id="footer">
        <footer>
            <div class="row first">
            <img src="~assets/hold.svg" alt="hold my drink" id="holdMyDrink"/>
            <img src="~assets/fuckoff-hand.png" alt="Fuck off" id="FuckOff"/>
            </div>
            <div class="row" id="footer_links">
                <div class="col-2">
                    <a href="#" alt="" class="footer__links">Terms and services</a>
                </div>
                <div class="col-2">
                    <a href="#" alt="" class="footer__links">Privacy policy</a>
                </div>
                <div class="col-2">
                    <a href="#" alt="" class="footer__links">About us</a>
                </div>
                <div class="col-2">
                    <a href="#" alt="" class="footer__links">Blog</a>
                </div>                
                <div class="col-2">
                     <a href="#" alt="" class="footer__links">Contact</a>
                </div>
            </div>
            <div class="row" id="footer__icons">
                <a href="" class="social-icons"><img src="~assets/twitter.png" alt="Twitter" tile="Twitter" class="social-icons__img"></a>
                <a href="" class="social-icons"><img src="~assets/discord.png" alt="Discord" tile="Discord" class="social-icons__img"></a>
                <a href="" class="social-icons"><img src="~assets/youtube.png" alt="Youtube"  tile="Youtube" class="social-icons__img"></a>
                <a href="" class="social-icons"><img src="~assets/medium.png" alt="Medium"  tile="Medium" class="social-icons__img"></a>
                <a href="" class="social-icons"><img src="~assets/instagram.png" alt="Instagram"  title="Instagram" class="social-icons__img"></a>

            </div>
        </footer>
    </div>
</template>
<script>
    export default {
    }
</script>

<style lang="scss" scoped>
.first{
    position: relative;
    height: 70vh;
    padding: 5rem;
}
#holdMyDrink{
    position: absolute;
    left:0;
    right:0;
    margin: auto;
    width: 90%;
}
#FuckOff{
    position: absolute;
    left: 0;
    bottom: 0;

}
.social-icons{
    position: relative;
    background:  linear-gradient(142.45deg, rgba(34, 34, 34) 4.24%, rgba(22, 22, 22, 0) 115.83%);
    mix-blend-mode: normal;
    border-radius: 2px;
    height: 54px;
    width: 54px;
    margin: 3rem;
}
.social-icons__img{
    position:  absolute;
    top: 50%;
    transform: translateY(-50%);
    left: 0;
    right: 0;
    margin: auto;
}
footer{
    background: black;
    position: absolute;
    bottom: 0;
    width: 100%;
    font-family: Titillium Web;
    font-style: normal;
    font-weight: 300;
    font-size: 21px;
    line-height: 40px;
    padding-bottom: 50px;
}
footer a{    
    color: #F4F4F4;
    text-decoration: none;
}
.row{
    text-align: center;
    place-content: center;
}
#footer_links{
    margin-bottom: 4rem;
        width: 70vw;
    margin: 5rem auto;
}
#footer__icons{
        width: 70vw;
    margin: auto;
}
</style>