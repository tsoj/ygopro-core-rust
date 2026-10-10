//! Evil★Twins Ki-sikil & Lil-la
use super::prelude::*;

pub const ID: u32 = 62098216;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KI_SIKIL, SET_LIL_LA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Cannot be Special Summoned except by procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon by tributing 2 Link monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Make opponent send cards to GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.tgtg)
    effect::set_target(d, e4, Some(cost_target(tgtg)));
    // e4:SetOperation(s.tgop)
    effect::set_operation(d, e4, Some(operation(tgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Gain ATK/DEF with Kiskil and Lilla in GY
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e5, EFFECT_UPDATE_ATTACK);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.atkcon)
    effect::set_condition(d, e5, Some(condition(atkcon)));
    // e5:SetValue(2200)
    effect::set_value(d, e5, 2200);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e6, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// function s.rfilter(c,tp) return c:IsLinkMonster() and (c:IsControler(tp) or c:IsFaceup()) end
fn rfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_link_monster(d, c) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),s.rfilter,2,false,2,true,c,nil,nil,nil,nil,c:GetControler())
    let tp = card::get_controler(d, c);
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: Some(2), check_field: true, to_check: Some(c), ..Default::default() };
    duel::check_release_group(d, tp, Some(&|d, x| rfilter(d, x, tp)), 2, ra, NO_EX, false)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,c)
// (the 9th argument is the `chk` value 1 of the target call, so the Lua `c` is the number 1)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None);
    // local g=Duel.SelectReleaseGroup(tp,s.rfilter,2,2,false,true,true,c,nil,nil,false,nil,tp)
    let ra = duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, ..Default::default() };
    let g = duel::select_release_group(d, tp, Some(&|d, x| rfilter(d, x, tp)), 2, 2, ra, NO_EX, false);
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
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let e = a.e(0);
    let g = effect::get_label_object(d, e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup() does nothing in Rust (groups are values)
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>2 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 2;
    }
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // local ct=#g-2
    let ct = group::get_count(&g) as i64 - 2;
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(1-tp,ct,ct,nil)
        let sg = group::select(d, &g, 1 - tp, ct as u16, ct as u16, false, NO_EX).nil();
        // Duel.SendtoGrave(sg,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, sg, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
    }
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(e:GetHandler():GetControler(),LOCATION_GRAVE,0)
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    let g = duel::get_field_group(d, p, LOCATION_GRAVE, 0);
    // return g:IsExists(Card.IsSetCard,1,nil,SET_KI_SIKIL) and g:IsExists(Card.IsSetCard,1,nil,SET_LIL_LA)
    group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_KI_SIKIL as u16]), 1, NO_EX)
        && group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_LIL_LA as u16]), 1, NO_EX)
}
