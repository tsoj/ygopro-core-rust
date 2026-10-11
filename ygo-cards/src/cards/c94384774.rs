//! Ebisu Shinsen Matsuri
use super::prelude::*;

pub const ID: u32 = 94384774;
/// `local TOKEN_SHINSEN=id+1`
const TOKEN_SHINSEN: u32 = ID + 1;
/// `local TOKEN_EBISU=id+2`
const TOKEN_EBISU: u32 = ID + 2;
/// `s.regspgroup` (the script table's field, kept in the duel)
const KEY_REGSPGROUP: u64 = 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SHINSEN, TOKEN_EBISU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 "Shinsen Token" (Aqua/WATER/Level 3/ATK 0/DEF 0) to its controller's opponent's field in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetTarget(s.shinsensptg)
    effect::set_target(d, e1, Some(cost_target(shinsensptg)));
    // e1:SetOperation(s.shinsenspop)
    effect::set_operation(d, e1, Some(operation(shinsenspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of Special Summoned Effect Monsters
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s.regspgroup=Group.CreateGroup()
        set_s_var(d, ID, KEY_REGSPGROUP, Value::Group(CardSet::new()));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        // ge1:SetOperation(s.regop)
        effect::set_operation(d, ge1, Some(operation(regop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Destroy both this card and as many Tokens on the field as possible, and if you do, Special Summon 1 "Ebisu Token" (Fairy/WATER/Level 7/ATK ?/DEF ?)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_BATTLE_START)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_BATTLE_START);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(function(e) return not e:GetHandler():HasFlagEffect(id+1) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.ebisusptg)
    effect::set_target(d, e2, Some(cost_target(ebisusptg)));
    // e2:SetOperation(s.ebisuspop)
    effect::set_operation(d, e2, Some(operation(ebisuspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.regspgroup`
fn regspgroup(d: &Duel) -> CardSet {
    s_var(d, ID, KEY_REGSPGROUP).as_group().cloned().unwrap_or_default()
}
/// `s.efftgfilter(c,e)`
fn efftgfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsEffectMonster() and c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and (not e or c:IsCanBeEffectTarget(e))
    card::is_effect_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && match e {
            None => true,
            Some(e) => card::is_can_be_effect_target(d, c, Some(e)),
        }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)` (ge1)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local tg=eg:Filter(s.efftgfilter,nil)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, x| efftgfilter(d, x, None), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // if Duel.GetCurrentChain()==0 then s.regspgroup:Clear() end
        let mut sg = regspgroup(d);
        if duel::get_current_chain(d, false) == 0 {
            sg = CardSet::new();
        }
        // s.regspgroup:Merge(tg)
        group::add_card(&mut sg, &tg);
        // s.regspgroup:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut sg, &|d, x| !card::has_flag_effect(d, x, ID, None), NO_EX);
        set_s_var(d, ID, KEY_REGSPGROUP, Value::Group(sg.clone()));
        // Duel.RaiseEvent(s.regspgroup,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_event(d, sg, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.sptgfilter(c,tp)`
fn sptgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local target_player=1-c:GetControler()
    let target_player = 1 - card::get_controler(d, c);
    // return Duel.GetLocationCount(target_player,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SHINSEN,0,TYPES_TOKEN,0,0,3,RACE_AQUA,ATTRIBUTE_WATER,POS_FACEUP_DEFENSE,target_player)
    duel::get_location_count(d, target_player, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SHINSEN, shinsen_data(), Some(POS_FACEUP_DEFENSE as u8), Some(target_player), 0)
}
/// The Shinsen Token (`Duel.IsPlayerCanSpecialSummonMonster` with `0` as the set code)
fn shinsen_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(3),
        race: Some(RACE_AQUA),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.shinsensptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn shinsensptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=s.regspgroup:Filter(s.efftgfilter,nil,e):Match(s.sptgfilter,nil,tp)
    let mut g = group::filter(d, &regspgroup(d), &|d, x| efftgfilter(d, x, Some(a.e)), NO_EX);
    group::match_(d, &mut g, &|d, x| sptgfilter(d, x, a.tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.efftgfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && efftgfilter(d, chkc, Some(a.e));
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local tc=nil
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        // tc=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    match tc {
        Some(tc) => duel::set_target_card(d, CardSet::from_card(tc)),
        None => duel::set_target_card(d, CardSet::new()),
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.shinsenspop(e,tp,eg,ep,ev,re,r,rp)`
fn shinsenspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local target_player=1-tc:GetControler()
        let target_player = 1 - card::get_controler(d, tc);
        // if Duel.GetLocationCount(target_player,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SHINSEN,0,TYPES_TOKEN,0,0,3,RACE_AQUA,ATTRIBUTE_WATER,POS_FACEUP_DEFENSE,target_player) then
        if duel::get_location_count(d, target_player, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_SHINSEN, shinsen_data(), Some(POS_FACEUP_DEFENSE as u8), Some(target_player), 0)
        {
            // local token=Duel.CreateToken(tp,TOKEN_SHINSEN)
            let token = duel::create_token(d, a.tp, TOKEN_SHINSEN).nil();
            // Duel.SpecialSummon(token,0,tp,target_player,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, token, 0, a.tp, target_player, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.ebisusptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ebisusptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,TYPE_TOKEN)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_type(d, x, TYPE_TOKEN)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 and Duel.GetMZoneCount(tp,g)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_EBISU,0,TYPES_TOKEN,-2,-2,7,RACE_FAIRY,ATTRIBUTE_WATER) end
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_EBISU, ebisu_data(-2), None, None, 0);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
    card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g+c,#g+1,tp,0)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(gc), g.len() as u8 + 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// The Ebisu Token: Fairy/WATER/Level 7, ATK and DEF `v` (`Duel.IsPlayerCanSpecialSummonMonster` with `0` as the set code)
fn ebisu_data(v: i32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(v),
        defense: Some(v),
        level: Some(7),
        race: Some(RACE_FAIRY),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `function(e) return not e:GetHandler():HasFlagEffect(id+1) end` (e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    !card::has_flag_effect(d, c, ID + 1, None)
}
/// `s.ebisuspop(e,tp,eg,ep,ev,re,r,rp)`
fn ebisuspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsType,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,TYPE_TOKEN)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_type(d, x, TYPE_TOKEN)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local des_ct=Duel.Destroy(g+c,REASON_EFFECT)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    let des_ct = duel::destroy(d, gc, REASON_EFFECT, None, None) as i64;
    // local atk_def_val=(des_ct-1)*700
    let atk_def_val = ((des_ct - 1) * 700) as i32;
    // if des_ct>=2 and Duel.GetOperatedGroup():IsContains(c)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_EBISU,0,TYPES_TOKEN,atk_def_val,atk_def_val,7,RACE_FAIRY,ATTRIBUTE_WATER) then
    if des_ct >= 2
        && group::is_contains(&duel::get_operated_group(d), c)
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_EBISU, ebisu_data(atk_def_val), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_EBISU)
        let token = duel::create_token(d, a.tp, TOKEN_EBISU).nil();
        // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // This Token's ATK/DEF become the number of Tokens destroyed by this effect x 700
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetValue(atk_def_val)
            effect::set_value(d, e1, atk_def_val as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // token:RegisterEffect(e2)
            card::register_effect(d, token, e2, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
