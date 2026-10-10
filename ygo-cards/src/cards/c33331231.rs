//! Surgical Striker - H.A.M.P.
use super::prelude::*;

pub const ID: u32 = 33331231;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER, SET_SKY_STRIKER_ACE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure (your field)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure (your opponent's field)
    // local e2=aux.AddLavaProcedure(c,0,POS_FACEUP,aux.TRUE,0,aux.Stringid(id,1))
    let e2 = aux::add_lava_procedure(d, c, Some(0), POS_FACEUP, Some(any_card()), Some(0), Some(aux::stringid(ID, 1)));
    // e2:SetCondition(s.hspcon2)
    effect::set_condition(d, e2, Some(lua_fn(hspcon2)));
    // e2:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e2, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.TRUE` as a card filter
fn any_card() -> CardFilter {
    std::sync::Arc::new(|_d, _c| true)
}
/// `s.ssacechk(e)`
fn ssacechk(d: &mut Duel, e: EffectId) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_SKY_STRIKER_ACE),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let tp = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SKY_STRIKER_ACE as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),aux.TRUE,1,false,1,true,c,c:GetControler(),nil,false,nil) and s.ssacechk(e)
    let tp = card::get_controler(d, c);
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), use_oppo: false, ..Default::default() };
    duel::check_release_group(d, tp, Some(&|_d, _c| true), 1, ra, NO_EX, false) && ssacechk(d, e)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)` (no `chk` parameter: the core passes the number 1 as `c`, so `a.card(8)` is nil)
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(8);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    // local g=Duel.SelectReleaseGroup(tp,aux.TRUE,1,1,false,true,true,c,nil,nil,false,nil)
    let ra = duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: c, use_oppo: false, ..Default::default() };
    let g = duel::select_release_group(d, tp, Some(&|_d, _c| true), 1, 1, ra, NO_EX, false);
    // if g then
    if let Some(g) = g {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, a.e(0)) else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup()
}
/// `s.hspcon2(e,c)`
fn hspcon2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return aux.LavaCondition(1,nil)(e,c) and s.ssacechk(e)
    let lava = aux::lava_condition(1, any_card());
    lava.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() && ssacechk(d, e)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
