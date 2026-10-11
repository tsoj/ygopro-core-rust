//! Rock Scales
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 57946551;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip 1 other face-up monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e1, 1, ID, 1, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsFaceup() end,Card.EquipByEffectAndLimitRegister,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a: Args| card::is_faceup(d, a.c(0))),
        lua_fn(|d, a: Args| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e1,
        None,
        None,
        None,
    );
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e2, 1, ID, 2, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,c)
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and tc:IsFaceup() and tc:IsMonster() then
    if card::is_relate_to_effect(d, tc, e)
        && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::is_faceup(d, tc)
        && card::is_monster(d, tc)
    {
        // e:GetHandler():EquipByEffectAndLimitRegister(e,tp,tc)
        let h = effect::get_handler(d, e).nil();
        card::equip_by_effect_and_limit_register(d, h, e, tp, tc, None, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:GetFlagEffect(id)>0
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER) && card::get_flag_effect(d, c, ID) > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=c:GetBattleTarget()
    let tc = card::get_battle_target(d, c);
    // if chk==0 then
    if a.chk == Some(0) {
        // local ec=c:GetEquipGroup():Filter(s.desfilter,nil):GetFirst()
        let eqg = card::get_equip_group(d, c);
        let fg = group::filter(d, &eqg, &|d, x| desfilter(d, x), NO_EX);
        let ec = group::get_first(d, &fg);
        // return ec and tc and tc:IsControler(1-tp) and tc:IsFaceup()
        //     and tc:IsAttribute(ec:GetOriginalAttribute()) and tc:IsRace(ec:GetOriginalRace())
        let (Some(ec), Some(tc)) = (ec, tc) else {
            return false;
        };
        return card::is_controler(d, tc, 1 - tp)
            && card::is_faceup(d, tc)
            && card::is_attribute(d, tc, card::get_original_attribute(d, ec))
            && card::is_race(d, tc, card::get_original_race(d, ec));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    let tc = card::get_battle_target(d, h).nil();
    // if tc:IsRelateToBattle() and tc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, tc) && card::is_controler(d, tc, 1 - tp) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
