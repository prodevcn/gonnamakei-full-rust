import {Injectable} from "@angular/core";
import {ActivatedRouteSnapshot, CanActivate, Router, RouterStateSnapshot, UrlTree} from "@angular/router";
import {Observable} from "rxjs";
import {first} from "rxjs/operators";

import {DataService} from "../services/data";
import {GMIWallets} from "../services/wallet";
import {ModalController} from "../components/modal";
import {WalletListComponent} from "../components/wallet-list/wallet-list.component";
import {sleep} from "../services/utils";

@Injectable({
    providedIn: "root",
})
export class ChallengeDetailGuard implements CanActivate {

    constructor(private gmiWallets: GMIWallets, private modalController: ModalController, private router: Router,
                private data: DataService) {
    }

    canActivate(route: ActivatedRouteSnapshot,
                state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {
        if (this.gmiWallets.connectedWallet) {
            return true;
        } else {
            return new Promise<boolean>((res) => {
                sleep(300).then(() => {
                    if (this.gmiWallets.connectedWallet) {
                        res(true);
                        return;
                    }

                    const {
                        closeModal,
                        close,
                    } = this.modalController.open({component: WalletListComponent});
                    closeModal.subscribe(() => {
                        const connected = !!this.gmiWallets.connectedWallet;

                        if (!connected && this.data.routePaths.length === 0) {
                            this.router.navigate(["/"]);
                        }

                        res(connected);
                    });

                    this.gmiWallets.onConnect.pipe(first()).subscribe(() => {
                        close();
                    });
                });
            });
        }
    }
}
