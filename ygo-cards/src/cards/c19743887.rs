//! VARefar, the Judge of Ball
use super::prelude::*;

pub const ID: u32 = 19743887;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand, then you can reveal 1 card in your hand and apply an effect based on the type of card revealed
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_ATKCHANGE+CATEGORY_REMOVE)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    // e1a:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1a, EFFECT_TYPE_QUICK_O);
    // e1a:SetRange(LOCATION_HAND)
    effect::set_range(d, e1a, LOCATION_HAND);
    // e1a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1a, EVENT_CHAINING);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(s.spcon1)
    effect::set_condition(d, e1a, Some(condition(spcon1)));
    // e1a:SetTarget(s.sptg)
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    // e1a:SetOperation(s.spop)
    effect::set_operation(d, e1a, Some(operation(spop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1b, EVENT_BE_BATTLE_TARGET);
    // e1b:SetCondition(s.spcon2)
    effect::set_condition(d, e1b, Some(condition(spcon2)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
}
// function s.spcontgfilter(c,tp)
fn spcontgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
// function s.spcon1(e,tp,eg,ep,ev,re,r,rp)
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (rp==1-tp and re:IsMonsterEffect() and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)) then return false end
    if !(a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    // return trig_loc==LOCATION_MZONE and tg and tg:IsExists(s.spcontgfilter,1,nil,tp)
    if trig_loc[0].int() != LOCATION_MZONE as i64 {
        return false;
    }
    match tg[0].as_group() {
        Some(tg) => group::is_exists(d, tg, &|d, c| spcontgfilter(d, c, a.tp), 1, NO_EX),
        None => false,
    }
}
// function s.spcon2(e,tp,eg,ep,ev,re,r,rp)
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    // local bt=Duel.GetAttackTarget()
    // return at and bt and at:IsControler(1-tp) and bt:IsControler(tp)
    match (duel::get_attacker(d), duel::get_attack_target(d)) {
        (Some(at), Some(bt)) => card::is_controler(d, at, 1 - a.tp) && card::is_controler(d, bt, a.tp),
        _ => false,
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // local opp_mon=(Duel.CheckEvent(EVENT_BE_BATTLE_TARGET) and Duel.GetAttacker())
    //     or (re:GetHandler():IsRelateToEffect(re) and eg:GetFirst())
    //     or nil
    let mut opp_mon = if duel::check_event(d, EVENT_BE_BATTLE_TARGET).is_some() { duel::get_attacker(d) } else { None };
    if opp_mon.is_none() {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            opp_mon = group::get_first(d, &a.eg.clone().nil());
        }
    }
    // if opp_mon then
    if let Some(opp) = opp_mon {
        // e:SetLabelObject(opp_mon)
        effect::set_label_object(d, a.e, Value::Card(opp));
        // if not opp_mon:IsLinkMonster() then
        if !card::is_link_monster(d, opp) {
            // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,opp_mon,1,tp,POS_FACEUP_DEFENSE)
            duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(opp)), 1, a.tp, POS_FACEUP_DEFENSE as i32);
        }
        // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,opp_mon,1,tp,opp_mon:GetAttack())
        let atk = card::get_attack(d, opp);
        duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(opp)), 1, a.tp, atk);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,opp_mon,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(opp)), 1, a.tp, 0);
    }
    true
}
// function s.revealfilter(c,types)
fn revealfilter(d: &mut Duel, c: CardId, types: u32) -> bool {
    // return c:IsType(types) and not c:IsPublic()
    card::is_type(d, c, types) && !card::is_public(d, c)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local opp_mon=e:GetLabelObject()
        // if not opp_mon then return end
        let Some(opp) = effect::get_label_object(d, a.e).as_card() else { return };
        // if Duel.CheckEvent(EVENT_BE_BATTLE_TARGET) then
        if duel::check_event(d, EVENT_BE_BATTLE_TARGET).is_some() {
            // if not opp_mon:IsRelateToBattle() then return false end
            if !card::is_relate_to_battle(d, opp) {
                return;
            }
        } else if !card::is_relate_to_effect(d, opp, a.re.nil()) {
            // if not opp_mon:IsRelateToEffect(re) then return false end
            return;
        }
        // local types=TYPE_SPELL
        let mut types = TYPE_SPELL;
        // if opp_mon:IsCanChangePosition() and opp_mon:IsAttackPos() then types=types|TYPE_MONSTER end
        if card::is_can_change_position(d, opp) && card::is_attack_pos(d, opp) {
            types |= TYPE_MONSTER;
        }
        // if opp_mon:IsAbleToRemove() then types=types|TYPE_TRAP end
        if card::is_able_to_remove(d, opp, None, None, None) {
            types |= TYPE_TRAP;
        }
        // if Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_HAND,0,1,nil,types) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x: CardId| revealfilter(d, x, types)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
            // local rc=Duel.SelectMatchingCard(tp,s.revealfilter,tp,LOCATION_HAND,0,1,1,nil,types):GetFirst()
            let sel = duel::select_matching_card(d, a.tp, Some(&|d: &mut Duel, x: CardId| revealfilter(d, x, types)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            let rc = group::get_first(d, &sel);
            // if not rc then return end
            let Some(rc) = rc else { return };
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ConfirmCards(1-tp,rc)
            duel::confirm_cards(d, 1 - a.tp, rc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // local op=rc:GetMainCardType()
            let op = card::get_main_card_type(d, rc);
            // Duel.BreakEffect()
            duel::break_effect(d);
            if op == TYPE_MONSTER {
                // Change it to Defense Position
                // Duel.ChangePosition(opp_mon,POS_FACEUP_DEFENSE)
                duel::change_position(d, opp, POS_FACEUP_DEFENSE as u8, None, None, None, false);
            } else if op == TYPE_SPELL {
                // Its ATK becomes doubled
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_SET_ATTACK)
                effect::set_code(d, e1, EFFECT_SET_ATTACK);
                // e1:SetValue(opp_mon:GetAttack()*2)
                let atk = card::get_attack(d, opp);
                effect::set_value(d, e1, atk as i64 * 2);
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // opp_mon:RegisterEffect(e1)
                card::register_effect(d, opp, e1, false);
            } else if op == TYPE_TRAP {
                // Banish it
                // Duel.Remove(opp_mon,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, opp, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
