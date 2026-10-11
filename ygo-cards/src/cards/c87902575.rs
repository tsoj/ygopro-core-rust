//! Future Visions
use super::prelude::*;

pub const ID: u32 = 87902575;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.clear)
    effect::set_target(d, e1, Some(cost_target(clear)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local ng=Group.CreateGroup()
    // ng:KeepAlive()
    // e1:SetLabelObject(ng)
    // (the group is kept in the label object; every change is written back with ng_set)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // return
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.retcon)
    effect::set_condition(d, e3, Some(condition(retcon)));
    // e3:SetOperation(s.retop)
    effect::set_operation(d, e3, Some(operation(retop)));
    // e3:SetLabelObject(e1)
    effect::set_label_object(d, e3, Value::Effect(e1));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_PHASE_START|PHASE_MAIN1)
    effect::set_code(d, e4, EVENT_PHASE_START | PHASE_MAIN1);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetOperation(s.clearop)
    effect::set_operation(d, e4, Some(operation(clearop)));
    // e4:SetLabelObject(e1)
    effect::set_label_object(d, e4, Value::Effect(e1));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}

/// The group kept in the label object of the Activate effect `e1` (`e1:GetLabelObject()`)
fn ng_get(d: &Duel, e1: EffectId) -> CardSet {
    effect::get_label_object(d, e1).as_group().cloned().nil()
}

/// Writes the group back into the label object of the Activate effect `e1`
fn ng_set(d: &mut Duel, e1: EffectId, g: CardSet) {
    effect::set_label_object(d, e1, Value::Group(g));
}

/// `e:GetLabelObject()` of an effect whose label object is the Activate effect
fn activate_of(d: &Duel, e: EffectId) -> EffectId {
    effect::get_label_object(d, e).as_effect().nil()
}

/// `s.clear(e,tp,eg,ep,ev,re,r,rp,chk)`
fn clear(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetLabelObject():Clear()
    ng_set(d, a.e, CardSet::new());
    true
}

/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:GetFirst():IsOnField() and eg:GetFirst():IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        let tc = group::get_first(d, &eg).nil();
        return card::is_on_field(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    // Duel.SetTargetCard(eg:GetFirst())
    let tc = group::get_first(d, &eg).nil();
    duel::set_target_card(d, tc);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg), 1, 0, 0);
    true
}

/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsRelateToEffect(e) and tc:IsLocation(LOCATION_MZONE) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_location(d, tc, LOCATION_MZONE)
        && duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0
    {
        // tc:CreateRelation(e:GetHandler(),RESET_EVENT|RESETS_STANDARD)
        card::create_relation(d, tc, c, RESET_EVENT | RESETS_STANDARD);
        // e:GetLabelObject():GetLabelObject():AddCard(tc)
        let e1 = activate_of(d, a.e);
        let mut g = ng_get(d, e1);
        group::add_card(&mut g, &CardSet::from_card(tc));
        ng_set(d, e1, g);
    }
}

/// `s.retfilter(c,ec,tp)`
fn retfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // return c:IsRelateToCard(ec) and c:IsPreviousControler(tp)
    card::is_relate_to_card(d, c, ec) && card::is_previous_controler(d, c, tp)
}

/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():GetLabelObject()
    let e1 = activate_of(d, a.e);
    let g = ng_get(d, e1);
    let c = effect::get_handler(d, a.e).nil();
    let tp = duel::get_turn_player(d);
    // return g:FilterCount(s.retfilter,nil,e:GetHandler(),Duel.GetTurnPlayer())>0
    group::filter_count(d, &g, &|d, x| retfilter(d, x, c, tp), NO_EX) > 0
}

/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // local lg=e:GetLabelObject():GetLabelObject()
    let e1 = activate_of(d, a.e);
    let mut lg = ng_get(d, e1);
    // local g=lg:Filter(s.retfilter,nil,e:GetHandler(),p)
    let mut g = group::filter(d, &lg, &|d, x| retfilter(d, x, c, p), NO_EX);
    // lg:Sub(g)
    group::remove_card(&mut lg, &g);
    ng_set(d, e1, lg);
    // local ft=Duel.GetLocationCount(p,LOCATION_MZONE)
    let ft = duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0;
    // if #g>ft then
    if g.len() as i32 > ft {
        // local sg=g
        let mut sg = g.clone();
        // Duel.Hint(HINT_SELECTMSG,p,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, p, aux::stringid(ID, 2));
        // g=g:Select(p,ft,ft,nil)
        let n = ft.max(0) as u16;
        g = group::select(d, &g, p, n, n, false, NO_EX).nil();
        // sg:Sub(g)
        group::remove_card(&mut sg, &g);
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do Duel.ReturnToField(tc,POS_FACEUP_ATTACK) end
    for tc in group::iter(d, &g) {
        duel::return_to_field(d, tc, Some(POS_FACEUP_ATTACK as u8), None);
    }
}

/// `s.clfilter(c,ec,tp)`
fn clfilter(d: &mut Duel, c: CardId, ec: CardId, tp: u8) -> bool {
    // return (not c:IsRelateToCard(ec)) or c:IsPreviousControler(tp)
    !card::is_relate_to_card(d, c, ec) || card::is_previous_controler(d, c, tp)
}

/// `s.clearop(e,tp,eg,ep,ev,re,r,rp)`
fn clearop(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():GetLabelObject():Remove(s.clfilter,nil,e:GetHandler(),Duel.GetTurnPlayer())
    let e1 = activate_of(d, a.e);
    let c = effect::get_handler(d, a.e).nil();
    let p = duel::get_turn_player(d);
    let mut lg = ng_get(d, e1);
    group::remove(d, &mut lg, &|d, x| clfilter(d, x, c, p), NO_EX);
    ng_set(d, e1, lg);
}
