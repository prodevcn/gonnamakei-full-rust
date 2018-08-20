import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DevnetMessageComponent } from './devnet-message.component';

describe('DevnetMessageComponent', () => {
  let component: DevnetMessageComponent;
  let fixture: ComponentFixture<DevnetMessageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DevnetMessageComponent ]
    })
    .compileComponents();
  });

  beforeEach(() => {
    fixture = TestBed.createComponent(DevnetMessageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
