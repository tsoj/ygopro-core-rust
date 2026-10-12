//! Diabellze the Mistress of the White Crow Forest
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 101403034;

pub const INFO: ScriptInfo = ScriptInfo { miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 Fusion or Synchro Monster + 1 Spellcaster or Illusion monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsType,TYPE_FUSION|TYPE_SYNCHRO),aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER|RACE_ILLUSION))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_FUSION | TYPE_SYNCHRO, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| {
                card::is_race_ctx(d, c, (RACE_SPELLCASTER | RACE_ILLUSION) as u64, Some(fc), st, Some(tp))
            })),
        ],
    );
    // If this card is Special Summoned: You can place any number of monsters you control face-up in their owners' Spell & Trap Zones as Continuous Spells, then you can return cards on the field to the hand up to the number of Spells/Traps you control. You can only use this effect of "Diabellze the Mistress of the White Crow Forest" once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 700 ATK/DEF for each Spell/Trap on the field
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2a, EFFECT_UPDATE_ATTACK);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetValue(function(e,c) return 700*Duel.GetMatchingGroupCount(Card.IsSpellTrap,0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil) end)
    effect::set_value_fn(
        d,
        e2a,
        lua_fn(|d: &mut Duel, _a: Args| {
            700 * duel::get_matching_group_count(d, Some(&|d, c| card::is_spell_trap(d, c)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64
        }),
    );
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // This face-up Fusion Summoned card cannot be used as Fusion Material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(
        d,
        e3,
        Some(lua_fn(|d: &mut Duel, a: Args| {
            let h = effect::get_handler(d, a.e(0)).nil();
            card::is_fusion_summoned(d, h)
        })),
    );
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // local owner=c:GetOwner()
    let owner = card::get_owner(d, c);
    // return Duel.GetLocationCount(owner,LOCATION_SZONE)>0 and c:CheckUniqueOnField(owner) and not c:IsForbidden()
    duel::get_location_count(d, owner, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::check_unique_on_field(d, c, owner, None, None)
        && !card::is_forbidden(d, c)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&plfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.rescon(your_max_selection,opp_max_selection)
//     return function(sg,e,tp,mg)
//         local your_g,opp_g=sg:Split(Card.IsOwner,nil,tp)
//         return #your_g<=your_max_selection and #opp_g<=opp_max_selection,#your_g>your_max_selection or #opp_g>opp_max_selection
//     end
// end
fn rescon(d: &mut Duel, sg: &CardSet, tp: u8, your_max_selection: usize, opp_max_selection: usize) -> (bool, bool) {
    let (your_g, opp_g) = group::split(d, sg, &|d, c| card::is_owner(d, c, tp), NO_EX);
    (
        your_g.len() <= your_max_selection && opp_g.len() <= opp_max_selection,
        your_g.len() > your_max_selection || opp_g.len() > opp_max_selection,
    )
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&plfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local your_monsters,opp_monsters=g:Split(Card.IsOwner,nil,tp)
    let (your_monsters, opp_monsters) = group::split(d, &g, &|d, c| card::is_owner(d, c, tp), NO_EX);
    // local your_max_selection=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let your_loc = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 as usize;
    // your_max_selection=math.min(#your_monsters,your_max_selection)
    let your_max_selection = your_monsters.len().min(your_loc);
    // local opp_max_selection=Duel.GetLocationCount(1-tp,LOCATION_SZONE)
    let opp_loc = duel::get_location_count(d, 1 - tp, LOCATION_SZONE as u8, None, None, None).0 as usize;
    // opp_max_selection=math.min(#opp_monsters,opp_max_selection)
    let opp_max_selection = opp_monsters.len().min(opp_loc);
    // local total_max_count=math.min(#g,your_max_selection+opp_max_selection)
    let total_max_count = g.len().min(your_max_selection + opp_max_selection);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,total_max_count,s.rescon(your_max_selection,opp_max_selection),1,tp,HINTMSG_TOFIELD)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(1),
        Some(total_max_count),
        Some(&|d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| {
            rescon(d, sg, tp, your_max_selection, opp_max_selection)
        }),
        1,
        tp,
        Some(HINTMSG_TOFIELD as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // local success=false
    let mut success = false;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // if not tc:IsImmuneToEffect(e) and Duel.MoveToField(tc,tp,tc:GetOwner(),LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        if !card::is_immune_to_effect(d, tc, e) {
            let owner = card::get_owner(d, tc);
            let is_monster = card::is_monster_card(d, tc);
            if duel::move_to_field(d, tc, tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, None) {
                // success=true
                success = true;
                // Treat it as a Continuous Spell
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
                // tc:RegisterEffect(e1)
                card::register_effect(d, tc, e1, false);
            }
        }
    }
    // if success and Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if success
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // local maxct=Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,LOCATION_ONFIELD,0,nil)
        let maxct = duel::get_matching_group_count(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, LOCATION_ONFIELD, 0, NO_EX) as u16;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local rthg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,maxct,nil)
        let rthg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, maxct, false, NO_EX).nil();
        // if #rthg>0 then
        if !rthg.is_empty() {
            // Duel.HintSelection(rthg)
            duel::hint_selection(d, rthg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(rthg,nil,REASON_EFFECT)
            duel::send_to_hand(d, rthg, None, REASON_EFFECT, None);
        }
    }
}
