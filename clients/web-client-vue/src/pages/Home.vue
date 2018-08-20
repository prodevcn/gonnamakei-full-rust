<template>
         <div class="top-container bg-color">
        <div class="container1">
            <div class="container-fluid">
                <div class="row pt-5">
                    <div class="col-md-6">
                        <div class="row">
                            <div class="col-md-3"></div>
                            <div class="col-md-9">
                                <p class="main-heading">
                                    <img alt="" class="img-fluid" src="~assets/main.svg">

                                    <span class="sub-heading">
			                        	<img src="~assets/images/main-subtext.svg">
			            			</span>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-6 notmobile pl-4">
                        <p class="f-right">
                            <img src="~assets/images/skeleton-king.svg">
                        </p>
                        <div class="row notmobile">
                            <div class="col-md-4"></div>
                            <div class="col-md-8">
                                <div class="bona2">
                                    <img alt="" class="img-fluid img2" src="~assets/degenape1.svg">
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 2nd row  -->

                <div class="row mt-4">

                    <div class="col-md-8 mt-4 notmobile">

                        <img alt="" class="img-fluid f-right p-4 m-4" src="~assets/thugbird.svg">
                        <img alt="" class="img-fluid f-right p-4 m-4" src="~assets/boogie.svg">
                        <img alt="" class="img-fluid f-right p-4 m-4" src="~assets/aurory.svg">
                    </div>
                    <div class="col-md-4 display-right p-0">

                        <!-- mobile  -->
                        <img alt="" class="img-fluid p-a s-a mobile " src="~assets/smb.svg">

                        <img alt="" class="img-fluid p-a t-a mobile " src="~assets/thugbird.svg">
                        <img alt="" class="img-fluid p-a b-a mobile " src="~assets/boogie.svg">
                        <img alt="" class="img-fluid p-a a-a mobile" src="~assets/aurory.svg">
                        <img alt="" class="img-fluid p-a d-a mobile" src="~assets/degenape1.svg">

                        <!-- destop  -->

                        <img alt="" class="img-fluid" src="~assets/Planet.svg">
                        <img alt="" class="img-fluid hand" src="~assets/hand_rock.svg">
                    </div>
                </div>

                <!-- 3rd row  -->
                <div class="row specific-height justify-content-center" style="padding-bottom: 150px">
                    <div class="row justify-content-center align-items-center text-white position-relative built-with-container">
                        <h5 class="title">Built on</h5>
                        <img alt="Solana" src="~assets/images/solana.svg"></div>
                </div>
            </div>
        </div>
<div class="container2">

            <div class="container-fluid">
                <div  class="container">
                    <div class="section-title text-center">
                        Challenge Markets
                    </div>
                    <div class="mt-4 mb-4">
                        <p class="ch text-center">
                            Create, play or invest in challenges </p>
                    </div>

                    <!-- <div class="devnet-message">
                        <devnet-message></devnet-message>
                    </div> -->

                    <!-- 1st row  -->
                    <div class="row mb-4 margin-top" style="align-items: stretch">
                        <!-- <div *ngFor="let challenge of availableChallenges" class="col-md-6 mt-4 mb-4 p-mobile">
                            <app-challenge [challenge]="challenge"></app-challenge>
                        </div> -->
                    </div>
                </div>

                <div class="row mt-4 center mb-4">

                    <button class="seeall mt-4" routerLink="/challenges">See All
                        <img src="~assets/icons/arrow-right2.svg"/>
                    </button>

                </div>
                <div class="row challenges-spacer height-spacer"></div>
            </div>
        </div>
                <div class="container2 bet-container">

            <div class="container-fluid">
                <div  class="container challenges-container">
                    <div class="section-title text-center">
                        <img alt="" class="img-fluid" src="~assets/MultiPlayerEngine.png">
                    </div>
                    <p class="ch text-center">
                        Bet whatever you want against whoever you want however you want </p>
                    <div class="competitors-container">
                        <div class="competitorA">
                            <img class="avatar" src="~assets/images/monkeydao1.png">
                            <div class="name">MonkeyDAO</div>
                            <div class="top-background"></div>
                            <div class="shadow-background"></div>
                        </div>
                        <div class="versus">
                            <img src="~assets/VS.png">
                        </div>
                        <div class="competitorB">
                            <img class="avatar" src="~assets/images/thugbird1.png">
                            <div class="name">ThugDAO</div>
                            <div class="top-background"></div>
                            <div class="shadow-background"></div>
                        </div>
                    </div>

                    <div class="action-container text-center">
                        <button  class="button">
                            Coming soon 🤙🏽
                        </button>
                    </div>

                </div>
                <div class="row height-spacer"></div>
            </div>
        </div>
<div class="container2">

            <div class="container-fluid">
                <div class="container play2mint-container">
                    <div class="section-title text-center">
                        <img alt="" class="img-fluid" src="~assets/PLAY2Mint.png">
                    </div>
                    <p class="ch text-center">
                        Play and get a random one </p>
                    <!-- 1st row  -->
                    <div  class="row mb-4 challenges-list-container" style="align-items: stretch">
                        <!-- <div *ngFor="let challenge of challenges">
                            <app-challenge2 [challenge]="challenge"></app-challenge2>
                        </div> -->
                    </div>

                    <div class="pagination-container">
                        <button  class="button icon-button">
                            <svg-icon src="~assets/icons/arrow-left.svg"></svg-icon>
                        </button>
                        <button  class="button icon-button">
                            <svg-icon src="~assets/icons/arrow-right.svg"></svg-icon>
                        </button>
                    </div>

                </div>
                <div class="row height-spacer"></div>
            </div>
        </div>
        <div class="container2 bet-container bet-container-background">

            <div class="container-fluid">
                <div  class="container challenges-container handy-container" >
                    <div class="section-title text-center">
                        <img alt="" class="img-fluid" src="~assets/HANDY_GAME.png">
                    </div>
                    <p class="ch text-center">
                        Use the Challenge Markets, Multiplayer Engine and Play2mint capabilities to build your own
                        experiences/games </p>
                    <div class="hands-container">
                        <img class="hand hand2" src="~assets/images/hand4.png">
                        <img class="hand hand1" src="~assets/images/hand5.png">
                        <img class="hand hand3" src="~assets/images/hand6.png">
                    </div>
                    <div class="background"></div>
                    <div class="action-container text-center">
                        <button  class="button">
                            Coming soon 🤙🏽
                        </button>
                    </div>

                </div>
                <div class="row height-spacer"></div>
            </div>
        </div>
     </div>
        <q-dialog v-model="showWalletConnector">
            <q-card class="bg-gmi-dark-light">
                <q-card-section>
                    <wallet-connector/>
                </q-card-section>
            </q-card>
        </q-dialog>

</template>

<script lang="ts">
    import {computed, defineComponent, ref} from "vue";
import {useI18n} from "vue-i18n";
import {Wallets} from "boot/wallets";
import WalletConnector from "components/WalletConnector.vue";
import {GMIRouter} from "src/router";

export default defineComponent({

});
</script>

<style lang="scss" scoped>
$color-1: #2e2e2e;
$color-2: #101010;
$color-3: #1b1b1b;
$color-4: #dedede;
$color-5: #9e9e9e;
$color-6: #b9b9b9;
$color-7: #e1ff28;
$color-8: #f22fb0;
$color-9: #93ab01;
$color-10: #28f2ff;
$color-11: #c4c4c4;
  .top-container.bg-color {
  background-image: linear-gradient(-160deg, transparent, #1b1b1b),
  url("~assets/mobile.png");
  opacity: 0.98;
}

.challenges-container {
  padding-top: 1rem;
}

.container1 {
    padding-top:10vh;
  .main-heading {
    display: inline-block;
    position: relative;
  }

  .sub-heading {
    left: 57%;
    position: absolute;
    top: 72%;

  }

  .f-right {
    float: right;
  }

  .bona2 {
    position: relative;
  }

  .img2 {
    position: absolute;
    top: 6rem;
  }

  .display-right {
    display: flex;
    justify-content: flex-end;
    position: relative;
  }

  .hand {
    position: absolute;

  }

  .mobile {
    display: none;
  }

  .specific-height {
    height: 5rem;
  }

  @media screen and (max-width: 600px) {
    .sub-heading {
      left: 9px;
      position: inherit;
      top: 26px;
    }
    .notmobile {
      display: none;
    }
    .mobile {
      display: block;
    }
    .p-a {
      height: 4rem;
      left: 0;
      position: absolute;
      width: 4rem;
    }
    .b-a {
      left: -2rem;
      top: 11rem;
    }
    .t-a {
      left: 4rem;
      top: 11rem;
    }
    .a-a {
      bottom: 2rem;
      left: 2rem;
    }
    .d-a {
      left: 6rem;
      top: 5rem;

    }
    .s-a {
      left: 13rem;
    }
  }

}

.container2 {
  .profile-img {
    border-radius: 50%;
    width: 66px;

  }

  .name {
    color: #ffffff;
    font-family: Titillium Web;
    font-size: 16px;
    font-style: normal;
    font-weight: 600;
    /* identical to box height, or 175% */
    letter-spacing: 0.03em;
    line-height: 28px;

    /* White */

    margin: 0;
  }

  .date {
    color: #dedede;
    font-family: Titillium Web;
    font-size: 12px;
    font-style: normal;
    font-weight: 300;
    /* identical to box height, or 233% */

    letter-spacing: 0.03em;

    /* Black/Black 100 */

    line-height: 28px;
    margin: 0
  }

  .card-bg {
    background: #2e2e2e;
  }

  .details {
    color: #dedede;
    font-family: Titillium Web;
    font-size: 14px;
    font-style: normal;
    font-weight: 300;
    /* or 186% */

    /* Black/Black 100 */

    line-height: 26px;

  }

  .m-img {
    margin-left: -2rem;
    margin-top: -2rem;
  }

  .margin-top {
    margin-top: 5.5rem;
  }

  .play {
    background: #e1ff28;
    border: none;
    float: right;
    height: 44px;
    width: 78px;
  }

  .gamer {
    color: #9e9e9e;
    font-family: Titillium Web;
    font-size: 12px;
    font-style: normal;
    font-weight: 300;
    /* identical to box height, or 233% */

    letter-spacing: 0.03em;

    line-height: 28px;
  }

  .padding-left {
    padding-left: 1.5rem;
  }

  .padding-right {
    padding-right: 1.5rem;
  }

  .pl-4 {
    padding-left: 1.5rem;
  }

  .seeall {
    background: transparent;
    border: none;
    color: #e1ff28;
    font-family: Titillium Web;
    font-size: 24px;
    /* identical to box height, or 117% */

    font-style: normal;
    font-weight: 300;
    letter-spacing: 0.03em;

    line-height: 28px;
  }

  .specific-height {
    height: 10rem;
  }

  .center {
    justify-content: center;
  }

  .ch {
    color: #b9b9b9;
    font-family: Titillium Web;
    font-size: 24px;
    font-style: normal;
    font-weight: 600;
    /* identical to box height, or 150% */

    line-height: 36px;
    margin: auto;
    max-width: 50rem;
  }

  .p-mobile {
    padding: 1.5rem 2rem;
  }

  .mobile {
    display: none;
  }

  @media screen and (max-width: 600px) {
    .sub-heading {
      left: 9px;
      position: inherit;
      top: 26px;
    }
    .notmobile {
      display: none !important;
    }
    .mobile {
      display: block;
    }
    .p-mobile {
      padding: 0;
    }
    .row-re {
      position: relative;
    }
    .img-ab {
      height: 77px;
      position: absolute;
      right: 1rem;
      top: -1rem;
      width: 77px;
    }
  }

  .main-p {
    color: #ab9e9e;
    font-size: 15px;
    padding: 26px;
  }

}

.section-title {
  color: white;
  font-family: Titillium Web;
  font-size: 4.75rem;
  font-weight: 400;
  margin: 0rem auto;
  margin-bottom: 1rem;
  text-transform: uppercase;
}

.competitors-container {
  align-items: center;
  display: flex;
  justify-content: center;
  margin: 8rem auto;

  > .versus {
    align-items: center;
    background: radial-gradient(40.03% 40.03% at 49.72% 50.28%, #28ff7e 0%, rgba(36, 255, 123, 0.272765) 60.21%, rgba(34, 255, 122, 0) 100%);
    color: var(--color-3);
    display: flex;
    justify-content: center;
    margin: auto 1rem;
    margin-right: 0;
    padding: calc(4rem + 25px) 4rem;
  }

  > .competitorB {
    transform: rotate(15deg) !important;
  }

  > .competitorA, .competitorB {
    align-items: center;
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow: visible;
    padding: 4rem;
    padding-bottom: 2rem;
    position: relative;
    transform: rotate(-15deg);

    > .name {
      color: white;
      font-size: 1.1rem;
      margin-top: 2rem;
      opacity: 1;
      z-index: 1;
    }

    > .avatar {
      z-index: 1;
    }

    > .shadow-background {
      background: rgba(242, 47, 176, 0.2);
      border: 0.1rem solid #f22fb0;
      border-radius: 0.2rem;
      box-sizing: border-box;
      height: 100%;
      left: 1.5rem;
      opacity: 0.8;
      position: absolute;
      top: 1.5rem;
      width: 100%;
      z-index: -1;
    }

    > .top-background {
     background: $color-1;
      border: 0.1rem solid var(--color-8);
      border-radius: 4px;
      box-sizing: border-box;
      height: 100%;
      left: 0rem;
      opacity: 0.8;
      position: absolute;
      top: 0rem;
      width: 100%;
    }

  }
}

.action-btn-container {
}

.pagination-container {
  display: flex;
  justify-content: center;

  > .button {
    margin-left: 0.5rem;
  }
}

.bet-container {
  overflow: hidden;
}

.play2mint-container {
  max-width: initial;

  .challenges-list-container {
    flex-wrap: nowrap;
    overflow: auto;
    -ms-overflow-style: none; /* IE and Edge */
    padding: 2rem;
    padding-top: 3rem;
    scrollbar-width: none; /* Firefox */

    &::-webkit-scrollbar {
      display: none;
    }

    > * {
      padding-right: 2.5rem;

      &:last-child {
        padding-right: 0;
      }
    }
  }
}

.height-spacer {
  height: 10rem;
}

.bet-container-background {
  background-image: url(~assets/images/World.svg);
  background-position: center;
  background-repeat: no-repeat;
  background-size: auto calc(100% - 5rem);
}

[handy-container] {
  position: relative;

  .background {
    background: radial-gradient(43.35% 43.35% at 49.72% 50.28%, rgba(40, 242, 255, 0.3) 0%, rgba(40, 242, 255, 0.114) 57.84%, rgba(40, 242, 255, 0) 100%);
    bottom: -3vw;
    height: 40rem;
    left: calc(50% - 20rem);
    position: absolute;
    width: 40rem;
  }

  .action-container {
    margin-top: 6rem;
    position: relative;
    z-index: 1;
  }
}

.hands-container {
  align-items: center;
  display: flex;
  justify-content: space-around;
  margin-top: 4rem;
  position: relative;
  z-index: 1;

  .hand {
    height: auto;
    max-width: 250px;
    width: 35%;
  }
}

.built-with-container {
  align-items: center !important;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 0.2rem;
  display: flex;
  padding: 1.1rem 1.8rem;
    margin: auto;
  > .title {
    margin: auto;
    margin-right: 1.2rem;
  }

  > img {
    max-height: 1.4rem;
    width: auto;
  }
}

@media screen and (max-width: 995px) {

  .section-title {
    font-size: 3.75rem;
  }

  .bet-container {
    background-size: 100% auto;
  }

  .height-spacer {
    height: 5rem;
  }

  .competitors-container {
    > .versus {
      padding: calc(2rem + 11px) 2rem;

      img {
        width: 50px;
      }
    }

    > .competitorA, .competitorB {
      min-width: 40%;
      padding: 2rem;
      padding-bottom: 1rem;
      width: 40%;

      > .avatar {
        width: 100%;
      }
    }
  }

  [handy-container] {
    .background {
      bottom: 0vw;
      height: 60vw;
      left: calc(50% - 30vw);
      width: 60vw;
    }
  }

}

@media screen and (max-width: 767px) {

  .section-title {
    font-size: 3.25rem;
  }

  [handy-container] {
    .background {
      bottom: 5.5rem;
    }
  }

  .challenges-spacer {
    height: 2rem !important;
  }

  .competitors-container {
    > .competitorA {
      transform: rotate(-15deg) translateY(-3rem) !important;
    }

    > .competitorB {
      transform: rotate(15deg) translateY(3rem) !important;
    }
  }
}

@media screen and (max-width: 500px) {

  .section-title {
    font-size: 2.5rem;
  }

}

.devnet-message {
  text-align: center;
  margin: 20px 0;
  display: flex;
  justify-content: center;
}

</style>