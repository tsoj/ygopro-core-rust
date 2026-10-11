//! Multiplying Kuriboh!
use super::prelude::*;

pub const ID: u32 = 14965712;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a monster effect, or an opponent's monster declares an attack (Quick Effect): You can Special Summon this card from your hand
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON);
    // e1a:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1a, EFFECT_TYPE_QUICK_O);
    // e1a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1a, EVENT_CHAINING);
    // e1a:SetRange(LOCATION_HAND)
    effect::set_range(d, e1a, LOCATION_HAND);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() end)
    effect::set_condition(d, e1a, Some(condition(|d, a| a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()))));
    // e1a:SetTarget(s.selfsptg)
    effect::set_target(d, e1a, Some(cost_target(selfsptg)));
    // e1a:SetOperation(s.selfspop)
    effect::set_operation(d, e1a, Some(operation(selfspop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1b, EVENT_ATTACK_ANNOUNCE);
    // e1b:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1b, Some(condition(|d, a| card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp))));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Once per turn, when your opponent activates a monster effect on the field, or during damage calculation, when an opponent's monster attacks (Quick Effect): You can add to your hand, or Special Summon, 1 "Dark Magician" or 1 monster with 300 ATK/200 DEF from your Deck or GY, then you can change that opponent's monster's ATK to 0
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2a, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    // e2a:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2a, EFFECT_TYPE_QUICK_O);
    // e2a:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2a, EVENT_CHAINING);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2a, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp and re:IsMonsterEffect() and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)&LOCATION_MZONE>0 end)
    effect::set_condition(
        d,
        e2a,
        Some(condition(|d, a| {
            a.ep == 1 - a.tp
                && effect::is_monster_effect(d, a.re.nil())
                && (duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() & LOCATION_MZONE as i64) > 0
        })),
    );
    // e2a:SetTarget(s.thsptg)
    effect::set_target(d, e2a, Some(cost_target(thsptg)));
    // e2a:SetOperation(s.thspop)
    effect::set_operation(d, e2a, Some(operation(thspop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2b, EVENT_PRE_DAMAGE_CALCULATE);
    // e2b:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e2b, Some(condition(|d, a| card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp))));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
// s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
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
    true
}
// s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// s.thspfilter(c,e,tp,mmz_chk)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return (c:IsCode(CARD_DARK_MAGICIAN) or (c:IsAttack(300) and c:IsDefense(200))) and (c:IsAbleToHand()
    //     or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_code(d, c, &[CARD_DARK_MAGICIAN]) || (card::is_attack(d, c, &[300]) && card::is_defense(d, c, &[200])))
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp,mmz_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// s.thspop(e,tp,eg,ep,ev,re,r,rp)
fn thspop(d: &mut Duel, a: &EffArgs) {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thspfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp,mmz_chk):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk))),
        a.tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // local res=aux.ToHandOrElse(sc,tp,
    //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //     function() return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
    //     aux.Stringid(id,3)
    // )
    let res = aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
        Some(&|d, g: &CardSet| duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 3)),
    );
    // if res==0 then return end
    if res == 0 {
        return;
    }
    // if sc:IsLocation(LOCATION_HAND) then Duel.ShuffleHand(tp) end
    if card::is_location(d, sc, LOCATION_HAND) {
        duel::shuffle_hand(d, a.tp);
    }
    // local opp_card=nil
    // local relation_chk=false
    let (opp_card, relation_chk) = if !duel::is_damage_step(d, None) {
        // if not Duel.IsDamageStep() then
        //     opp_card=re:GetHandler()
        //     relation_chk=opp_card:IsRelateToEffect(re)
        let re = a.re.nil();
        let opp = effect::get_handler(d, re).nil();
        let rel = card::is_relate_to_effect(d, opp, re);
        (opp, rel)
    } else {
        // else
        //     opp_card=Duel.GetAttacker()
        //     relation_chk=opp_card:IsRelateToBattle()
        let opp = duel::get_attacker(d).nil();
        let rel = card::is_relate_to_battle(d, opp);
        (opp, rel)
    };
    // if relation_chk and opp_card:IsFaceup() and opp_card:IsControler(1-tp) and not opp_card:IsAttack(0)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
    if relation_chk
        && card::is_faceup(d, opp_card)
        && card::is_controler(d, opp_card, 1 - a.tp)
        && !card::is_attack(d, opp_card, &[0])
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 4))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Change that opponent's monster's ATK to 0
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // opp_card:RegisterEffect(e1)
        card::register_effect(d, opp_card, e1, false);
    }
}
