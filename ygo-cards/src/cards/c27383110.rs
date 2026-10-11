//! 宣告者の預言
//! Dawn of the Herald
use super::prelude::*;

pub const ID: u32 = 27383110;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={44665365}
    listed_names: &[44665365],
    // s.fit_monster={44665365}
    fit_monster: &[44665365],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.AddProcEqual(c,s.ritualfil,6,nil,nil,nil,nil,s.stage2)
    let e1 = ritual::add_proc_equal(
        d,
        c,
        ritual::RitualProc {
            filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| ritualfil(d, x))),
            lv: Some(ritual::Lv::N(6)),
            stage2: Some(std::sync::Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            ..Default::default()
        },
    );
    // salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(s.thcost)
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // event
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e3, EVENT_CHAIN_END);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCondition(s.evcon)
    effect::set_condition(d, e3, Some(condition(evcon)));
    // e3:SetOperation(s.evop)
    effect::set_operation(d, e3, Some(operation(evop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // e1:SetLabelObject(e3)
    effect::set_label_object(d, e1, Value::Effect(e3));
}
/// `s.ritualfil(c)`
fn ritualfil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(44665365) and c:IsRitualMonster()
    card::is_code(d, c, &[44665365]) && card::is_ritual_monster(d, c)
}
/// `s.stage2(mg,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    // e:GetLabelObject():SetLabelObject(tc)
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label_object(d, e3, Value::Card(tc));
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetHandler()==e:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    let c = effect::get_handler(d, a.e).nil();
    rc == c
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_location(d, c, LOCATION_GRAVE) && card::is_able_to_remove(d, c, None, None, None);
    }
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsControler(tp) and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_location(d, c, LOCATION_GRAVE) && card::is_controler(d, c, tp) && card::is_able_to_hand(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local mat=tc:GetMaterial()
    let mat = card::get_material(d, tc);
    // if chkc then return mat:IsContains(chkc) and s.thfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&mat, chkc) && thfilter(d, chkc, e, tp);
    }
    // if chk==0 then return mat:IsExists(s.thfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &mat, &|d, x| thfilter(d, x, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=mat:FilterSelect(tp,s.thfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &mat, tp, &|d, x| thfilter(d, x, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
/// `s.evcon(e,tp,eg,ep,ev,re,r,rp)`
fn evcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject()~=nil
    !effect::get_label_object(d, a.e).is_nil()
}
/// `s.evop(e,tp,eg,ep,ev,re,r,rp)`
fn evop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // Duel.RaiseEvent(tc,EVENT_CUSTOM+id,e,0,tp,tp,0)
    duel::raise_event(d, tc, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, e, Value::Nil);
}
