//! Maluth the White Crow Knight
use super::prelude::*;

pub const ID: u32 = 101403010;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_WHITE_CROW],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can send 1 other card from your hand or field to the GY; place this card face-up in its owner's Spell & Trap Zone as a Continuous Spell, and if you do, Special Summon up to 2 "White Crow" monsters with different names from each other from your Deck, except "Maluth the White Crow Knight"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(placeandspcost)));
    effect::set_target(d, e1, Some(cost_target(placeandsptg)));
    effect::set_operation(d, e1, Some(operation(placeandspop)));
    card::register_effect(d, c, e1, false);
    // When a monster declares an attack and this card is a Continuous Spell: You can target 1 "White Crow" Monster Card in your Spell & Trap Zone; Special Summon it
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        // return e:GetHandler():IsContinuousSpell()
        let h = effect::get_handler(d, a.e).nil();
        card::is_continuous_spell(d, h)
    })));
    effect::set_target(d, e2, Some(cost_target(stzonesptg)));
    effect::set_operation(d, e2, Some(operation(stzonespop)));
    card::register_effect(d, c, e2, false);
}
// s.costfilter(c,handler_mmz_chk,stzone_chk,tp,handler)
fn costfilter(d: &mut Duel, c: CardId, handler_mmz_chk: bool, stzone_chk: bool, tp: u8, handler: CardId) -> bool {
    // return c:IsAbleToGraveAsCost() and ((c:IsLocation(LOCATION_STZONE) and handler_mmz_chk)
    //   or (stzone_chk and Duel.GetMZoneCount(tp,Group.FromCards(handler,c))>0))
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    if card::is_location(d, c, LOCATION_STZONE) && handler_mmz_chk {
        return true;
    }
    if !stzone_chk {
        return false;
    }
    let mut two = CardSet::from_card(handler);
    two.insert(c);
    duel::get_m_zone_count(d, tp, (&two).into(), None, None, None).0 > 0
}
// s.placeandspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn placeandspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    // local handler_mmz_chk=Duel.GetMZoneCount(tp,c)>0
    let handler_mmz_chk = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0;
    // local stzone_chk=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let stzone_chk = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,c,handler_mmz_chk,stzone_chk,tp,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| costfilter(d, x, handler_mmz_chk, stzone_chk, tp, c)),
            tp,
            LOCATION_HAND | LOCATION_ONFIELD,
            0,
            1,
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c,handler_mmz_chk,stzone_chk,tp,c)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| costfilter(d, x, handler_mmz_chk, stzone_chk, tp, c)),
        tp,
        LOCATION_HAND | LOCATION_ONFIELD,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// s.deckspfilter(c,e,tp)
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WHITE_CROW) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WHITE_CROW as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.placeandsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn placeandsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    true
}
// s.placeandspop(e,tp,eg,ep,ev,re,r,rp)
fn placeandspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.MoveToField(c,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if card::is_relate_to_effect(d, c, e) && duel::move_to_field(d, c, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        // Treat this card as a Continuous Spell
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local max_count=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut max_count = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // if max_count<=0 then return end
        if max_count <= 0 {
            return;
        }
        // local g=Duel.GetMatchingGroup(s.deckspfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, x| deckspfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // max_count=math.min(max_count,#g,2)
        max_count = max_count.min(g.len() as i32).min(2);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then max_count=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            max_count = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,max_count,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let dn = aux::dncheck();
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(max_count as usize), Some(&dn), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default();
        // if #sg>0 then Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP) end
        if !sg.is_empty() {
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
// s.stzonespfilter(c,e,tp)
fn stzonespfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WHITE_CROW) and c:IsMonsterCard() and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_WHITE_CROW as u16])
        && card::is_monster_card(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.stzonesptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn stzonesptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_STZONE) and chkc:IsControler(tp) and s.stzonespfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_STZONE) && card::get_controler(d, chkc) == tp && stzonespfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.stzonespfilter,tp,LOCATION_STZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| stzonespfilter(d, c, e, tp)), tp, LOCATION_STZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.stzonespfilter,tp,LOCATION_STZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| stzonespfilter(d, c, e, tp)), tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
// s.stzonespop(e,tp,eg,ep,ev,re,r,rp)
fn stzonespop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, tc, e) {
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
