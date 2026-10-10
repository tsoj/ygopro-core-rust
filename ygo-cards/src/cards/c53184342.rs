//! Dragunity Arma Gram
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 53184342;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon procedure (from hand or GY)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate targeted monster's effects, also loses ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equip an opponent's monster destroyed by battle to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.eqpcon)
    effect::set_condition(d, e3, Some(condition(eqpcon)));
    // e3:SetTarget(s.eqptg)
    effect::set_target(d, e3, Some(cost_target(eqptg)));
    // e3:SetOperation(s.eqpop)
    effect::set_operation(d, e3, Some(operation(eqpop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsMonster),Card.EquipByEffectAndLimitRegister,e3)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_monster(d, a.c(0))),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)),
        e3,
        None,
        None,
        None,
    );
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WINGEDBEAST|RACE_DRAGON) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_WINGEDBEAST | RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let h = effect::get_handler(d, e).nil();
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,e:GetHandler())
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, h.into());
    let chkf = aux::chkf_mmz(1);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>1
    //     and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
            && rg.len() > 1
            && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&chkf), 0, tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&chkf), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.distg(c,eqpc)`
fn distg(d: &mut Duel, c: CardId, eqpc: bool) -> bool {
    // return c:IsNegatableMonster() or (eqpc and c:IsFaceup() and c:GetAttack()>0)
    card::is_negatable_monster(d, c) || (eqpc && card::is_faceup(d, c) && card::get_attack(d, c) > 0)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local eqpc=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),tp,LOCATION_ONFIELD,0,1,nil)
    let equip_faceup = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EQUIP));
    let eqpc = duel::is_existing_matching_card(d, Some(&equip_faceup), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.distg(chkc,eqpc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && distg(d, chkc, eqpc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.distg,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,eqpc) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| distg(d, c, eqpc)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,s.distg,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,eqpc)
    let g = duel::select_target(d, tp, Some(&|d, c| distg(d, c, eqpc)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // if tc:IsCanBeDisabledByEffect(e) then
    if card::is_can_be_disabled_by_effect(d, tc, e, true) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsType,TYPE_EQUIP),tp,LOCATION_SZONE,0,nil)
    let equip_faceup = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_EQUIP));
    let ct = duel::get_matching_group_count(d, Some(&equip_faceup), a.tp, LOCATION_SZONE, 0, NO_EX);
    // if ct>0 then
    if ct > 0 {
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetValue(-ct*1000)
        effect::set_value(d, e3, -(ct as i64) * 1000);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
/// `s.eqpfilter(c,tp)`
fn eqpfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_BATTLE)
    //     and c:IsPreviousControler(1-tp) and s.exfilter(c,tp)
    card::is_monster(d, c)
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_previous_controler(d, c, 1 - tp)
        && exfilter(d, c, tp)
}
/// `s.exfilter(c,tp)`
fn exfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return not c:IsForbidden() and c:CheckUniqueOnField(tp)
    !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None)
}
/// `s.eqpcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqpcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.eqpfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| eqpfilter(d, c, tp), 1, NO_EX)
}
/// `s.eqptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local eqg=eg:Filter(s.eqpfilter,nil,tp)
    let eqg = group::filter(d, &eg, &|d, c| eqpfilter(d, c, tp), NO_EX);
    // if chk==0 then return #eqg>0 and Duel.GetLocationCount(tp,LOCATION_SZONE)>=#eqg end
    if a.chk == Some(0) {
        return !eqg.is_empty() && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= eqg.len() as i32;
    }
    // Duel.SetTargetCard(eqg)
    duel::set_target_card(d, eqg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,eqg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(eqg), 1, 0, 0);
    true
}
/// `s.eqlimit(e,c)`: `return e:GetOwner()==c`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    effect::get_owner(d, e) == Some(c)
}
/// `s.eqpop(e,tp,eg,ep,ev,re,r,rp)`
fn eqpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<1 then return end
    if ft < 1 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tg=Duel.GetTargetCards(e):Filter(s.exfilter,nil,tp)
    let targets = duel::get_target_cards(d, e).unwrap_or_default();
    let tg = group::filter(d, &targets, &|d, x| exfilter(d, x, tp), NO_EX);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and #tg>0 and ft>0 then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, e) && !tg.is_empty() && ft > 0 {
        // local eqg=nil
        // if #tg>ft then eqg=tg:Select(tp,ft,ft,nil)
        // else eqg=tg end
        let eqg = if tg.len() as i32 > ft {
            group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil()
        } else {
            tg
        };
        // for tc in eqg:Iter() do
        for tc in group::iter(d, &eqg) {
            // if Duel.Equip(tp,tc,c,true,true) then
            if duel::equip(d, tp, tc, c, true, true) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT+EFFECT_FLAG_OWNER_RELATE)
                effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT + EFFECT_FLAG_OWNER_RELATE, 0);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // e1:SetValue(s.eqlimit)
                effect::set_value_fn(d, e1, lua_fn(eqlimit));
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
    }
}
