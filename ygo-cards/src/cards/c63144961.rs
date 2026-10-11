//! Majestic Absorption
use super::prelude::*;

pub const ID: u32 = 63144961;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function s.tgfilter(c,tp,ft)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32) -> bool {
    // local eq=ft>0 and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    let eq = ft > 0 && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local datk=not c:IsHasEffect(EFFECT_DIRECT_ATTACK) and Duel.IsAbleToEnterBP()
    let datk = card::is_has_effect(d, c, EFFECT_DIRECT_ATTACK, None).is_empty() && duel::is_able_to_enter_bp(d);
    // local dam=Duel.IsAbleToEnterBP()
    let dam = duel::is_able_to_enter_bp(d);
    // return c:IsFaceup() and (c:IsCode(CARD_STARDUST_DRAGON) or (c:ListsCode(CARD_STARDUST_DRAGON) and c:IsType(TYPE_SYNCHRO)))
    //     and (eq or datk or dam)
    card::is_faceup(d, c)
        && (card::is_code(d, c, &[CARD_STARDUST_DRAGON])
            || (card::lists_code(d, c, &[CARD_STARDUST_DRAGON]) && card::is_type(d, c, TYPE_SYNCHRO)))
        && (eq || datk || dam)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp, ft);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| tgfilter(d, cc, tp, ft)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,ft):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, cc| tgfilter(d, cc, tp, ft)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local b1=ft>0 and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    let b1 = ft > 0 && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b2=not tc:IsHasEffect(EFFECT_DIRECT_ATTACK) and Duel.IsAbleToEnterBP()
    let b2 = card::is_has_effect(d, tc, EFFECT_DIRECT_ATTACK, None).is_empty() && duel::is_able_to_enter_bp(d);
    // local b3=Duel.IsAbleToEnterBP()
    let b3 = duel::is_able_to_enter_bp(d);
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    );
    // (not cancelable: the selection is always made)
    let op = op.map_or(0, |x| x as i64);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_EQUIP)
        effect::set_category(d, a.e, CATEGORY_EQUIP);
        // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 1 - tp, LOCATION_MZONE as i32);
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if op == 1 && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::is_controler(d, tc, tp)
        && card::is_faceup(d, tc)
    {
        // Equip 1 face-up monster your opponent controls to that target
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
        // local ec=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_faceup(d, cc)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let ec = match group::get_first(d, &sel) {
            Some(ec) => ec,
            None => return,
        };
        // if not tc:EquipByEffectAndLimitRegister(e,tp,ec,nil,true) then return false end
        if !card::equip_by_effect_and_limit_register(d, tc, e, tp, ec, None, Some(true)) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetValue(true)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
    } else if op == 2 && card::is_has_effect(d, tc, EFFECT_DIRECT_ATTACK, None).is_empty() {
        // It can attack directly this turn
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3205)
        effect::set_description(d, e2, 3205);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT+EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    } else if op == 3 && duel::is_able_to_enter_bp(d) {
        // Inflict damage each time it destroys a monster
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e3:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
        // e3:SetCondition(aux.bdocon)
        effect::set_condition(d, e3, Some(aux::bdocon()));
        // e3:SetOperation(function(_,tp) Duel.Damage(1-tp,tc:GetBattleTarget():GetTextAttack(),REASON_EFFECT) end)
        effect::set_operation(
            d,
            e3,
            Some(lua_fn(move |d, args: Args| {
                let tp2 = args.p(1);
                let bt = card::get_battle_target(d, tc).nil();
                let atk = card::get_text_attack(d, bt) as i64;
                duel::damage(d, 1 - tp2, atk, REASON_EFFECT, false, None);
            })),
        );
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
