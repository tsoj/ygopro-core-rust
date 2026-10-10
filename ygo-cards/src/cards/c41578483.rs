//! Millennium-Eyes Restrict
use super::prelude::*;

pub const ID: u32 = 41578483;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Fusion Materials
    // Fusion.AddProcMix(c,true,true,64631466,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(64631466),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), st, Some(tp)))),
        ],
    );
    // --Equip 1 Effect Monster the opponent controls or in their GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // --Gains ATK/DEF equal to the equipped monsters'
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():GetEquipGroup():FilterCount(s.eqgfilter,nil)>0 end)
    effect::set_condition(d, e2, Some(lua_fn(|d, a| {
        let h = effect::get_handler(d, a.e(0)).nil();
        let eg = card::get_equip_group(d, h);
        group::filter_count(d, &eg, &|d, c| eqgfilter(d, c), NO_EX) > 0
    })));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // e3:SetValue(s.defval)
    effect::set_value_fn(d, e3, effect_card_value(defval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Monsters with the equipped monsters' original name cannot attack
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e4, EFFECT_CANNOT_ATTACK);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e4, LOCATION_MZONE, LOCATION_MZONE);
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(effect_card_fn(distg)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Their effects on the field are negated
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e5, EFFECT_DISABLE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --Their activated effects are negated
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e6, EVENT_CHAIN_SOLVING);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCondition(s.discon)
    effect::set_condition(d, e6, Some(condition(discon)));
    // e6:SetOperation(function(_,_,_,_,ev) Duel.NegateEffect(ev) end)
    effect::set_operation(d, e6, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect()
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil())
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_change_controler(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.eqfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && eqfilter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.eqfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| eqfilter(d, c)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and s.eqfilter(tc) and tc:IsControler(1-tp) then
    if card::is_relate_to_effect(d, tc, a.e) && eqfilter(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.eqval(ec,c,tp)` (`c` is not used)
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(1-tp) and ec:IsType(TYPE_EFFECT)
    card::is_controler(d, ec, 1 - tp) && card::is_type(d, ec, TYPE_EFFECT)
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,id)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
}
/// `s.eqgfilter(c)`
fn eqgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:HasFlagEffect(id)
    card::is_faceup(d, c) && card::has_flag_effect(d, c, ID, None)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetEquipGroup():Match(s.eqgfilter,nil):Match(function(c) return c:GetTextAttack()>0 end,nil)
    let mut g = card::get_equip_group(d, c);
    group::match_(d, &mut g, &|d, x| eqgfilter(d, x), NO_EX);
    group::match_(d, &mut g, &|d, x| card::get_text_attack(d, x) > 0, NO_EX);
    // return g:GetSum(Card.GetTextAttack)
    group::get_sum(d, &g, &|d, x| card::get_text_attack(d, x) as i64)
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=c:GetEquipGroup():Match(s.eqgfilter,nil):Match(function(c) return c:GetTextDefense()>0 end,nil)
    let mut g = card::get_equip_group(d, c);
    group::match_(d, &mut g, &|d, x| eqgfilter(d, x), NO_EX);
    group::match_(d, &mut g, &|d, x| card::get_text_defense(d, x) > 0, NO_EX);
    // return g:GetSum(Card.GetTextDefense)
    group::get_sum(d, &g, &|d, x| card::get_text_defense(d, x) as i64)
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local eqg=e:GetHandler():GetEquipGroup():Match(s.eqgfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let mut eqg = card::get_equip_group(d, h);
    group::match_(d, &mut eqg, &|d, x| eqgfilter(d, x), NO_EX);
    // return eqg:IsExists(Card.IsOriginalCodeRule,1,nil,c:GetOriginalCodeRule())
    let codes = card::get_original_code_rule(d, c);
    group::is_exists(d, &eqg, &|d, x| card::is_original_code_rule(d, x, &codes), 1, NO_EX)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqg=e:GetHandler():GetEquipGroup():Match(s.eqgfilter,nil)
    let h = effect::get_handler(d, a.e).nil();
    let mut eqg = card::get_equip_group(d, h);
    group::match_(d, &mut eqg, &|d, x| eqgfilter(d, x), NO_EX);
    // return re:IsMonsterEffect() and eqg:IsExists(Card.IsOriginalCodeRule,1,nil,re:GetHandler():GetOriginalCodeRule())
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    let rh = effect::get_handler(d, a.re.nil()).nil();
    let codes = card::get_original_code_rule(d, rh);
    group::is_exists(d, &eqg, &|d, x| card::is_original_code_rule(d, x, &codes), 1, NO_EX)
}
